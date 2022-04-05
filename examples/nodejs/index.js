const Rollbar = require('rollbar-node')

const rollbar = new Rollbar({
  accessToken: process.env.POST_TOKEN,
})

rollbar.log('critical', 'oopsie', {
  some: 'stuff'
})
