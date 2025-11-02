# salomon

Framework modulare sviluppato in Rust.

## Comandi base

### Avvio dell'applicazione rust

Dentro la cartella `salomon` dare questo comando:
```bash
cargo run -p app
```

#### Guida online

Una volta avviato il server, la guida è disponibile all'url `http://localhost:8080/guide/`

N.B. Se la guida è stata modificata, nella root del progetto va dato il comando `mdbook build book` per buildare nuovamente e applicare le modifiche

### Avvio del frontend

N.B. Se è la prima volta, entro la cartella `salomon/frontend` dare questo comando: `npm install`, che scaricherà tutto il necessario

Dentro la cartella `salomon/frontend` dare questo comando:
```bash
npm run dev
```
Questo avvia la webapp Next.js, visualizzabile all'url: `http://localhost:3000/`