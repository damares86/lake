"use client";

import { useState } from "react";
import Image from "next/image";
import styles from "./page.module.css";
import Carousel from "./components/Carousel";

export default function Home() {
  const [message, setMessage] = useState("");

  // const handleInsert = async () => {
  //   try {
  //     const res = await fetch("http://localhost:8080/api/insert");
  //     const text = await res.text();
  //     setMessage(text);
  //   } catch (err) {
  //     setMessage("Errore nella richiesta al backend");
  //   }
  // };

  return (
    <div className={styles.container}>
      <main className={styles.main}>
        <Image
          className={styles.logo}
          src="/images/rustix-logo.png"
          alt="rustix logo"
          width={500}
          height={300}
          priority
        />
        {/* <button onClick={handleInsert} className={styles.button}>
          Insert test_key into TiKV
        </button>

        {message && <p className={styles.message}>{message}</p>} */}


        <Carousel />
      </main>

      <footer className={styles.footer}>
        {/* Footer links or content here */}
      </footer>
    </div>
  );
}