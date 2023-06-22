import "./style.css";

import { initDrop } from "./image_drop";

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <div class='dropBox' id="drop">Drop On Me</div>
  </div>
`;

initDrop();
