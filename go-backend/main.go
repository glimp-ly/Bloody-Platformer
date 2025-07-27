package main

import (
	"encoding/json"
	"log"
	"net/http"
)

type GameStats struct {
	Kills    int `json:"kills"`
	Deaths   int `json:"deaths"`
	Upgrades int `json:"upgrades"`
}

func main() {
	http.HandleFunc("/stats", handleStats)
	log.Println("Server running on :8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}

func handleStats(w http.ResponseWriter, r *http.Request) {
	// CORS Headers
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Content-Type", "application/json")

	// Handle preflight request
	if r.Method == http.MethodOptions {
		w.Header().Set("Access-Control-Allow-Methods", "POST, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type")
		w.WriteHeader(http.StatusNoContent)
		return
	}

	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var stats GameStats
	if err := json.NewDecoder(r.Body).Decode(&stats); err != nil {
		log.Printf("Error decoding JSON: %v", err)
		http.Error(w, "Invalid JSON format", http.StatusBadRequest)
		return
	}

	log.Printf("Stats received: %+v\n", stats)

	// Respond with confirmation
	json.NewEncoder(w).Encode(map[string]string{"status": "Stats received"})
}
