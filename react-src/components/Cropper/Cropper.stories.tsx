import React from "react";
import { ComponentStory, ComponentMeta } from "@storybook/react";
import Cropper from "./Cropper";

export default {
  title: "MuhaqiqFacsimileUtil/Cropper",
  component: Cropper,
} as ComponentMeta<typeof Cropper>;

const Template: ComponentStory<typeof Cropper> = (args) => (
  <Cropper {...args} />
);

export const HelloWorld = Template.bind({});
HelloWorld.args = {
  label: "Hello world!",
};

export const ClickMe = Template.bind({});
ClickMe.args = {
  label: "Editor!",
};
