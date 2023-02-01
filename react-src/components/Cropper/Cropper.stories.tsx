import React from "react";
import { ComponentStory, ComponentMeta } from "@storybook/react";
import Cropper from "./Cropper";

export default {
  title: "MuhaqiqFacsimileUtil/Cropper",
  component: Cropper,
  argTypes: {
    fileName: {
      options: ["P3471_11"],
      control: { type: "radio" },
    },
    color: {
      control: { type: "color" },
    },
    framePercentage: {
      control: { type: "number" },
    },
  },
} as ComponentMeta<typeof Cropper>;

const Template: ComponentStory<typeof Cropper> = (args) => (
  <Cropper {...args} />
);

export const ArabicHandwrittingSimpleExample = Template.bind({});
ArabicHandwrittingSimpleExample.args = {
  fileName: "P3471_11",
};
