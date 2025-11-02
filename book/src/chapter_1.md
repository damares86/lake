# Struttura di salomon

    salomon/
    ├── app/             
    ├── book/             
    ├── framework/       
    ├── frontend/        
    ├── Cargo.toml
    └── Config.toml

## framework

Il core, la base in cui si trovano tutti metodi che possono essere riutilizzati in più applicazioni e vanno inserite le librerie custom.

## app

La parte in cui si crea l'applicazione specifica, qui vanno messi tutti i metodi specifici della singola applicazione.

## frontend

Qui si trova la webapp in Next.js.

## book

La documentazione realizzata con [mdbook](https://rust-lang.github.io/mdBook/).

## File configurazione

### Cargo.toml

Qui vanno richiamate tutte le librerie necessarie al funzionamento del framework, che vengono poi buildate .

### Config.toml

Il file per le configurazioni del framework (es. endpoint db).