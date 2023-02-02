module.exports = {
  stories: [
    "../react-src/**/*.stories.mdx",
    "../react-src/**/*.stories.@(js|jsx|ts|tsx)",
  ],
  staticDirs: ["./static"],
  addons: [
    "@storybook/addon-links",
    "@storybook/addon-essentials",
    "@storybook/addon-interactions",
  ],
  framework: "@storybook/react",
  core: {
    builder: "@storybook/builder-webpack5",
  },
  async webpackFinal(config) {
    config.experiments = config.experiments
      ? { ...config.experiments, asyncWebAssembly: true }
      : { asyncWebAssembly: true };

    config.module.rules.push({
      resolve: { fullySpecified: false },
    });
    return config;
  },
};
