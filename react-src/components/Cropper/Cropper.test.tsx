import React from "react";
import { render } from "@testing-library/react";

import Cropper from "./Cropper";

describe("Cropper", () => {
  test("renders the Button component", () => {
    render(<Cropper label="Hello world!" />);
  });
});
