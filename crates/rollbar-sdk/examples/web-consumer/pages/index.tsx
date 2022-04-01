import type { NextPage } from 'next'
import Head from 'next/head'
import Image from 'next/image'
import { useEffect, useState, useCallback } from 'react'
import styles from '../styles/Home.module.css'

import { Instance } from 'rollbar-sdk'

const generateId = (() => {
  let counter = 0;
  return () => counter++;
})();

const defaultExtra = [
  { id: generateId(), key: 'an', value: 'example' }
];

const Home: NextPage = () => {
  const [rollbar, setRollbar] = useState();

  useEffect(() => {
    const instance = Instance.fromConfig({
        accessToken: 'b5938ecbdb984aa091234644b0686c3d'
    });

    setRollbar(instance);
  }, []);

  const [level, setLevel] = useState('debug');

  const [message, setMessage] = useState('Rust is cool.');

  const [extraFields, setExtraFields] = useState(defaultExtra);

  const handleExtraFieldChange = useCallback((id, next) => {
    setExtraFields(state => state.map((field) => field.id === id ? next : field));
  }, [setExtraFields]);

  const handleAddExtraField = useCallback(() => {
    setExtraFields(extraFields.concat({
      id: generateId(),
      key: '',
      value: '',
    }));
  }, [extraFields, setExtraFields]);

  const handleSendMessage = useCallback((id, next) => {
    if (!rollbar) {
        return;
    }

    const extra = extraFields
      .reduce((extra, { key, value }) => ({ ...extra, [key]: value }), {});

    rollbar[level](message, extra);
    // This also works:
    // rollbar.log(level, message, extraFields);
  }, [rollbar, level, message, extraFields]);

  const [randomWord, setRandomWord] = useState();

  useEffect(() => {
    fetch('/api/random')
      .then(res => res.json())
      .then(({ word }) => {
        setRandomWord(word);
      });
  }, []);

  return (
    <div className={styles.container}>
      <h1>Rollbar SDK... from Rust 🦀!</h1>

      <label>Level:{' '}</label>
      <select name="level" id="level" onChange={event => setLevel(event.target.value)} value={level}>
        {['debug', 'info', 'warning', 'error', 'critical']
          .map(value => <option key={value} value={value}>{value}</option>)}
      </select>
      <br />
      <label>Message:{' '}</label>
      <input value={message} onChange={event => setMessage(event.target.value)} />
      <br />
      <label>Extra:{' '}</label>
      <table>
        <thead>
          <tr>
            <td>Key</td>
            <td>Value</td>
          </tr>
        </thead>
        <tbody>
          {extraFields.map(({ id, key, value }) => (
            <tr key={id}>
              <td>
                <input
                  value={key}
                  onChange={event => handleExtraFieldChange(id, {
                    id,
                    key: event.target.value,
                    value
                  })}
                />
              </td>
              <td>
                <input
                  value={value}
                  onChange={event => handleExtraFieldChange(id, {
                    id,
                    key,
                    value: event.target.value
                  })}
                />
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <button onClick={handleAddExtraField}>Add extra field</button>
      <br />
      <button onClick={handleSendMessage}>Send it 🏂!</button>
      <h2>Now for node...</h2>
      <button>Fetch data from endpoint</button>
      <p>Random word from api endpoint: {}</p>
    </div>
  )
}

export default Home
