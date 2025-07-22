package main

import (
	"encoding/json"
	"log"
	"net/http"
)

type GameStats struct {
	Kills     int `json:"kills"`
	Deaths    int `json:"deaths"`
	Upgrades  int `json:"upgrades"`
}

func main() {
	http.HandleFunc("/stats", handleStats)
	log.Println("Server running on :8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}

func handleStats(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var stats GameStats
	if err := json.NewDecoder(r.Body).Decode(&stats); err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}

	log.Printf("Stats received: %+v\n", stats)
	w.WriteHeader(http.StatusCreated)
}