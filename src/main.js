import { invoke } from "@tauri-apps/api/core";
import { open, message } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";

const YouTubeRegex =
  /^((?:https?:)?\/\/)?((?:www|m)\.)?((?:youtube\.com|youtu.be))(\/(?:[\w\-]+\?v=|embed\/|v\/)?)([\w\-]+)(\S+)?$/;

(async () => {
  await listen("download-progress", (event) => {
    const progress = event.payload;
    document.getElementById("download-progress").value = progress;
    console.log("Progress:", progress + "%");
  });

  await listen("download-complete", async () => {
    console.log("Download complete!");
    document.getElementById("download-progress").value = 0;
    await message("Download complete!", { title: "Oxitube", kind: "info" });
  });

  document.getElementById("download").addEventListener("submit", async (e) => {
    e.preventDefault();
    document.getElementById("download-progress").value = 0;

    const formData = new FormData(e.target);
    const url = formData.get("url");
    const quality = formData.get("quality");
    const filter = formData.get("filter");

    if (!YouTubeRegex.test(url)) {
      await message("Please enter a valid URL", {
        title: "Error",
        kind: "error",
      });
      return;
    }

    const file_path = await open({
      directory: true,
      multiple: false,
    });

    if (file_path) {
      try {
        await invoke("download", {
          url: url,
          quality: quality,
          filter: filter,
          outFilePath: file_path,
        });
      } catch (error) {
        console.error("Download error:", error);
        await message(error, {
          title: "Download Failed",
          kind: "error",
        });
        document.getElementById("download-progress").value = 0;
      }
    }
  });
})();
