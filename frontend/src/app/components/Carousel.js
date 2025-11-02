"use client";

import { useEffect, useState } from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
} from "recharts";
import styles from "./Carousel.module.css";

export default function Carousel() {
  const [dataSets, setDataSets] = useState([]);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const res = await fetch("http://localhost:8080/api/data"); // <-- Assicurati che sia corretto
        if (!res.ok) throw new Error("Errore nel fetch");
        const json = await res.json();
        setDataSets(json);
      } catch (err) {
        console.error("❌ Errore nel caricamento dei dati:", err);
      }
    };

    fetchData();
  }, []);

  return (
    <div className={styles.carousel}>
      {dataSets.map((data, index) => (
        <div className={styles.card} key={index}>
          <h3 className={styles.title}>Dataset {index + 1}</h3>
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={data}>
              <XAxis dataKey="name" />
              <YAxis />
              <Tooltip />
              <Line
                type="monotone"
                dataKey="value"
                stroke="#8884d8"
                strokeWidth={2}
              />
            </LineChart>
          </ResponsiveContainer>
        </div>
      ))}
    </div>
  );
}
