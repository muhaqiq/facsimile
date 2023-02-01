import React from "react";

export interface CropperProps {
  label: string;
}

const Cropper = (props: CropperProps) => {
  return <h1>{props.label}</h1>;
};

export default Cropper;
// https://dev.to/kouts/deploy-storybook-to-github-pages-3bij
