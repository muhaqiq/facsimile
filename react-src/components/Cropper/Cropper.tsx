import React, { useCallback, useEffect, useState } from "react";
import { FacsimileCropper } from "../../util";
import { base46, hexToRgbUint32Array, loadImageAsDataUrl } from "../helpers";

export interface ICropperProps {
  fileName: string;
  color: string;
  framePercentage: number;
}

async function loadFacsimileCropper(url: string): Promise<FacsimileCropper> {
  const util = await import("../../util");
  const data = await loadImageAsDataUrl(url);
  return util.FacsimileCropper.new(base46(data));
}

const Cropper = ({ fileName, color, framePercentage }: ICropperProps) => {
  const [image, setImage] = useState<null | string>(null);
  const [wasmImg, setWasmImg] = useState<FacsimileCropper | null>(null);
  const [regions, setRegions] = useState<
    [Uint32Array, number, Uint32Array, string][]
  >([]);
  useEffect(() => {
    loadFacsimileCropper(`${fileName}.jpeg`)
      .then((imgCrp) => {
        setWasmImg(imgCrp);
        setImage(imgCrp.get_url());
      })
      .catch((err) => console.log(err));

    fetch(`${fileName}.json`)
      .then((res) => res.json())
      .then(({ TextElements }: { TextElements: any[] }) => {
        setRegions(
          TextElements.map(({ FacsimileRegion, Position }: any, i) => {
            const p = new Uint32Array(8);
            p[0] = FacsimileRegion.Points[0].X;
            p[1] = FacsimileRegion.Points[0].Y;
            p[2] = FacsimileRegion.Points[1].X;
            p[3] = FacsimileRegion.Points[1].Y;
            p[4] = FacsimileRegion.Points[2].X;
            p[5] = FacsimileRegion.Points[2].Y;
            p[6] = FacsimileRegion.Points[3].X;
            p[7] = FacsimileRegion.Points[3].Y;
            return [
              p,
              FacsimileRegion.Rotation,
              hexToRgbUint32Array("#000000"),
              Position,
            ];
          })
        );
      });
    return () => {
      if (wasmImg) {
        wasmImg.free();
      }
    };
  }, []);

  const getRegion = useCallback(
    (i: number) => {
      if (wasmImg) {
        const p = regions[i][0] as Uint32Array;
        const r = regions[i][1] as number;
        const transformedColor = hexToRgbUint32Array(color);
        setImage(wasmImg.get_region(p, r, transformedColor, framePercentage));
      }
    },
    [wasmImg, regions, framePercentage, color]
  );

  return (
    <div style={{ display: "flex", flexDirection: "column" }}>
      <div style={{ padding: "10px" }}>
        {regions.map((data, i) => (
          <button
            style={{ marginRight: "10px" }}
            key={i}
            onClick={() => getRegion(i)}
          >
            {data[3]}
          </button>
        ))}

        <button
          style={{ marginRight: "10px" }}
          onClick={() => wasmImg && setImage(wasmImg.get_url())}
        >
          Full Page
        </button>
      </div>
      {image && (
        <img
          style={{ border: "solid black 2px" }}
          width="500"
          height="auto"
          src={image}
          alt="failed"
        />
      )}
    </div>
  );
};

export default Cropper;
