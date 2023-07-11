import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";


let image;


function App() {


  async function draw_image() {
    const canvas = document.getElementById("canv") as HTMLCanvasElement;
    const ctx = canvas.getContext("2d");
    const imageData = document.getElementById("fileIn") as HTMLInputElement;
  
    if(imageData.files === null) {
      return
    }

    image = await createImageBitmap(imageData.files[0]);


    let modWidth = canvas.width / image.width;
    let modHeight = canvas.height / image.height;

    console.log(canvas.height / image.height);
    ctx?.clearRect(0, 0, canvas.width, canvas.height);
    ctx?.drawImage(image, 0, 0, image.width * modWidth, image.height * modHeight);
  }

  return (
    <div className="container">
      <nav>
        <h1>Image Converter</h1>
      </nav>
      <div id="Cont">
        <canvas id="canv"></canvas>
        <div id="formCont">
          <input type="file" onChange={draw_image} id="fileIn" accept="image/jpeg, image/png, image/jpg"></input>
          <label>Resolution X:</label>
          <input min="10" type="number" value={10}></input>
          <label>Resolution Y:</label>
          <input min="10" type="number" value={10}></input>
          <select>
            <option value="RGB-8">RGB 8-bit</option>
          </select>
          <input type="textfield" ></input>
        </div>
        <div className="ArrayCont">

        </div>
      </div>
    </div>
  );
}

export default App;
