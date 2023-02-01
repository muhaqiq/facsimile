export async function loadImageAsDataUrl(url: string) {
    const data: string = await fetch(url)
      .then((response) => response.blob())
      .then(
        (blob) =>
          new Promise((resolve) => {
            const reader = new FileReader();
            reader.onload = function () {
              resolve(this.result as string);
            };
            reader.readAsDataURL(blob);
          })
      );

    return data;
  }

  export const base46 = (data: string) =>
    data.replace(/^data:image\/(png|jpeg|jpg);base64,/, '');

    export function hexToRgbUint32Array(hex: string): Uint32Array {
      const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
      const { r, g, b } = result
        ? {
          r: parseInt(result[1], 16),
          g: parseInt(result[2], 16),
          b: parseInt(result[3], 16),
        }
        : { r: 0, g: 0, b: 0 };
      const arr = new Uint32Array(3);
      arr[0] = r;
      arr[1] = g;
      arr[2] = b;
      return arr
    }

    export const highlightColors = [
      '#6495ED',
      '#C0392B',
      '#8E44AD',
      '#16A085',
      '#FFFFC2',
      '#566573',
      '#2ECC71',
      '#0020C2',
      '#6F4E37',
      '#E77471',
      '#64E986',
      '#C8A2C8',
      '#D35400',
      '#7F525D',
    ];
