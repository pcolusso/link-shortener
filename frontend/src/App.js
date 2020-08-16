import React, { useState } from 'react'
import './App.css'

function App() {
  const [url, setUrl] = useState("")
  const [slug, setSlug] = useState("")
  const [msg, setMsg] = useState(null)
  const [newUrl, setNewUrl] = useState(null)

  const handleSubmit = async (evt) => {
    evt.preventDefault();
    const response = await fetch("/", {
      method: "POST",
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ slug: slug, url: url })
    })
    try {
      const json = await response.json()
      if (json.status === 'error') {
        setMsg(json.reason)
      } else {
        setMsg("URL enshortenified!")
        setNewUrl(`${window.location}${json.slug}`)
      }
    } catch (e) {
      setMsg("Something went wrong");
    }
  }

  return (
    <div className="App">
      <header className="App-header">
        <h1>link shortener</h1>
        <span>enter your link below. get a shorter link</span>
      </header>
      { msg && (
        <span>{msg}</span>
      )}
      <form onSubmit={handleSubmit}>
        <label>Slug
          <input type="text" value={slug} onChange={event => setSlug(event.target.value)} />
        </label>
        <label>URL
          <input type="text" value={url} onChange={event => setUrl(event.target.value)} />
        </label>
        <input type="submit" value="Shorten!" />
      </form>
      { newUrl && (
        <a href={newUrl}>{newUrl}</a>
      )}
    </div>
  );
}

export default App;
