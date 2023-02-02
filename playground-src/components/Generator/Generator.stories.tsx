import React from "react";
import { ComponentStory, ComponentMeta } from "@storybook/react";
import Generator from "./Generator";

export default {
  title: "MuhaqiqFacsimileUtil/Generator",
  argTypes: {
    width: {
      control: { type: "number" },
    },
    height: {
      control: { type: "number" },
    },
    backgroundColor: {
      control: { type: "color" },
    },
    gridColor: {
      control: { type: "color" },
    },
  },
  component: Generator,
} as ComponentMeta<typeof Generator>;

const Template: ComponentStory<typeof Generator> = (args) => (
  <Generator {...args} />
);

export const Playground = Template.bind({});
