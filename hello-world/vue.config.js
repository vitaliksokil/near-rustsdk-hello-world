const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  publicPath: process.env.VUE_APP_NODE_ENV === 'production'
      ? '/near-rustsdk-hello-world/'
      : '/',

  transpileDependencies: true,
  css: {
    loaderOptions: {
      scss: {
        // additionalData: `@import "~@/assets/scss/app.scss";`
      },
    }
  }
})
