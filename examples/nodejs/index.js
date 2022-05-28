const Rollbar = require('rollbar-node');

const rollbar = new Rollbar({
  accessToken: process.env.POST_TOKEN,
  codeVersion: "abc123",
  environment: "prod"
});

rollbar.log('warning', 'yeehaw', {
  some: 'stuff'
});

rollbar.shutdown();
