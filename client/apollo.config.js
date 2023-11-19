// apollo.config.js
module.exports = {
    client: {
      service: {
        name: 'giskard_test',
        // URL to the GraphQL API
        url: 'http://localhost:8000/',
      },
      // Files processed by the extension
      includes: [
        'src/**/*.vue',
        'src/**/*.ts'
      ],
    },
  }