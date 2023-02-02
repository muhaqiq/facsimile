import React, { useCallback, useState } from "react";
import { hexToRgbUint8Array } from "../helpers";

export interface IGeneratorProps {
  width?: number;
  height?: number;
  backgroundColor?: string;
  gridColor?: string;
}

export function Generator({
  width,
  height,
  backgroundColor,
  gridColor,
}: IGeneratorProps) {
  const [image, setImage] = useState<null | string>(null);

  const generate = useCallback(async () => {
    const { generate_facsimile } = await import("../../util");
    setImage(
      generate_facsimile(
        width,
        height,
        hexToRgbUint8Array(backgroundColor ?? "#E3DDCA"),
        hexToRgbUint8Array(gridColor ?? "000000")
      )
    );
  }, [width, height, gridColor, backgroundColor]);

  return (
    <div>
      <h3>
        This module was originally built to generate placeholders for facsimile
        scans for which appropriate licensing for display in public websites
        were not avaliable
      </h3>
      <div>
        <ul>
          <li>Width: {width ?? 210}</li>
          <li>Height: {height ?? 297}</li>
          <li>Background Color: {backgroundColor ?? "#E3DDCA"}</li>
          <li>Grid Color: {gridColor ?? "#000000"}</li>
        </ul>
        <button onClick={generate}>generate</button>
      </div>
      <br />
      {image && (
        <img
          src={image}
          style={{ border: "solid black 2px" }}
          width="500"
          height="auto"
          alt="loading"
        />
      )}
    </div>
  );
}

export default Generator;
