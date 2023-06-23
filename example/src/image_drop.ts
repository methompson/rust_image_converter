import wasm_heif from "@saschazar/wasm-heif";

import init, {
  process_image,
  process_heif_image,
} from "./pkg/rust_image_converter";

export async function initDrop() {
  await init();

  const drop = document.querySelector("#drop");

  console.log("drop?", drop);

  drop?.addEventListener("dragover", (ev) => {
    // prevent default to allow drop
    ev.preventDefault();
  });
  drop?.addEventListener("drop", (ev) => {
    ev.preventDefault();
    if (ev instanceof DragEvent) {
      readFile(ev);
      return;
    }
  });
}

function readFile(ev: DragEvent) {
  const files = ev.dataTransfer?.files ?? [];
  for (const item of files) {
    console.log("type:", item.type);
    if (item.type === "image/heic") {
      readHeif(item);
    } else {
      readFileAsBytes(item);
    }
  }
}

async function readFileAsBytes(file: File) {
  console.log(file instanceof File);
  const buf = await file.arrayBuffer();

  const uint8Arr = new Uint8Array(buf);

  try {
    const result = process_image(uint8Arr, { new_format: "jpeg" });
    console.log("result", result);
    makeFileFromBytes(result);
  } catch (e) {
    console.error(e);
  }
}

async function readHeif(file: File) {
  const heifModule = await wasm_heif();
  console.log(heifModule);

  const buff = await file.arrayBuffer();
  const uint8Arr = new Uint8Array(buff);

  const decodedValue = heifModule.decode(uint8Arr, uint8Arr.length, false);
  const dimensions = heifModule.dimensions();

  console.log("dimensions", dimensions);

  if (decodedValue instanceof Uint8Array) {
    console.log(decodedValue.length);
    console.log(uint8Arr.length);

    const result = process_heif_image(
      decodedValue,
      dimensions.width,
      dimensions.height,
      {
        max_size: 1000,
        new_format: "jpeg",
      }
    );

    makeFileFromBytes(result);
  }

  heifModule.free();
}

function makeFileFromBytes(bytesArray: Uint8Array) {
  console.log("Making File", bytesArray.length);
  // const b64encoded = arrayBufferToBase64(bytesArray);
  const blob = new Blob([bytesArray]);
  const blobUrl = URL.createObjectURL(blob);

  const downloadEl = document.createElement("a");
  downloadEl.setAttribute("href", blobUrl);
  downloadEl.setAttribute("download", "new_file.jpg");

  downloadEl.style.display = "none";
  document.body.appendChild(downloadEl);

  downloadEl.innerHTML = "link";

  downloadEl.click();

  document.body.removeChild(downloadEl);
}
