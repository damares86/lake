# Il framework

    framework/
    ├── config.rs             
    ├── db.rs             
    ├── frontend.rs
    ├── lib.rs      
    └── server.rs

## lib.rs

Punto di accesso del framework, si occupa di esportare le librerie.

## config.rs

Crea le strutture dati di base per la configurazione.

## server.rs

Le funzionalità di base per tirare su un server. Comprende il richiamo di `db.rs` per la connessione a TiKV.

## db.rs

I metodi per la connessione al db TiKV.

## frontend.rs

I metodi dedicati alla comunicazione tra backend e frontend.