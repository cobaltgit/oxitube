import { invoke } from "@tauri-apps/api/core";
import { open, message } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";

await listen("download-progress", (event) => {
  const progress = event.payload;
  document.getElementById("download-progress").value = progress;
  document.getElementById("progress-percent").innerText = `${progress}%`;
});

await listen("download-complete", async () => {
  document.getElementById("download-progress").value = 0;
  document.getElementById("progress-percent").innerText = "";
  await message("Download complete!", { title: "Oxitube", kind: "info" });
});

document.getElementById("download").addEventListener("submit", async (e) => {
  e.preventDefault();

  const formData = new FormData(e.target);
  const url = formData.get("url");
  const quality = formData.get("quality");
  const filter = formData.get("filter");

  const file_path = await open({
    directory: true,
    multiple: false,
  });

  if (file_path) {
    console.log("Selected directory:", file_path);
    await invoke("download", {
      url: url,
      quality: quality,
      filter: filter,
      outFilePath: file_path,
    });
  }
});
