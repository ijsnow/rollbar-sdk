// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import randomWord from 'random-words'

//import { Instance } from 'rollbar-sdk'

type Data = {
  name: string
}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  //const rollbar = Instance.fromConfig({
      //accessToken: 'b5938ecbdb984aa091234644b0686c3d'
  //});

  const word = randomWord() as string;

  //instance.info("a word was randomly generated", {
    //word
  //});
  
  res.status(200).json({ word })
}
