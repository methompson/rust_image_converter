import wasm_heif from "@saschazar/wasm-heif";
import init, {
  read_bytes,
  create_image_from_rgb,
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

  readFileFromBytes(uint8Arr);
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

    // readFileFromBytes(decodedValue);

    const result = create_image_from_rgb(
      decodedValue,
      dimensions.height,
      dimensions.width
    );

    makeFileFromBytes(result);
  }

  heifModule.free();
}

async function readFileFromBytes(dat: Uint8Array) {
  try {
    const result = read_bytes(dat);
    console.log("result", result);
  } catch (e) {
    console.error(e);
  }
}

function makeFileFromBytes(bytesArray: Uint8Array) {
  console.log("Making File", bytesArray.length);
  // const b64encoded = arrayBufferToBase64(bytesArray);
  const blob = new Blob([bytesArray]);
  const blobUrl = URL.createObjectURL(blob);

  const downloadEl = document.createElement("a");
  downloadEl.setAttribute("href", blobUrl);
  downloadEl.setAttribute("download", "new_file.bin");

  // downloadEl.style.display = "none";
  document.body.appendChild(downloadEl);

  downloadEl.innerHTML = "link";

  // downloadEl.click();

  // document.body.removeChild(downloadEl);
}

function arrayBufferToBase64(uint8Arr: Uint8Array) {
  console.log("array to buffer");
  let binary = "";
  const len = uint8Arr.byteLength;

  console.log("iterating");
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(uint8Arr[i]);
  }

  console.log("Done iterating");

  return btoa(binary);
}
