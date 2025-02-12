import { invoke } from "@tauri-apps/api/tauri";

let msgLastRefreshed: HTMLElement | null;

async function refreshBtn() {
  if (msgLastRefreshed) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    msgLastRefreshed.textContent = await invoke("refresh_data", {
      name: "Last refreshed data",
    });

    //reload images
    const images: NodeListOf<HTMLImageElement> = document.querySelectorAll("img");
    images.forEach((img: HTMLImageElement) => {
    img.src = img.src.split('?')[0] + "?" + new Date().getTime();
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  msgLastRefreshed = document.querySelector("#msg-last-refreshed");
  document.querySelector("#refresh-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    refreshBtn();
  });
});
