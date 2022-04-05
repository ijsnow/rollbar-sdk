const Rollbar = require('rollbar-node')

const rollbar = new Rollbar({
  accessToken: 'b5938ecbdb984aa091234644b0686c3d'
})

rollbar.log('critical', 'uh oh', {
  some: 'stuff'
})
  .then(() => {
    console.log('i think it worked')
  })
  .catch(() => {
    console.log('it did not work')
  })
