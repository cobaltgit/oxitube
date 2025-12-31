import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

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
