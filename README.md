# arnob-stellar-project - 🧠 Soroban Smart Contract (Event RSVP)
<img width="1919" height="919" alt="Screenshot 2026-03-27 133820" src="https://github.com/user-attachments/assets/a0436d31-bbb3-4203-b52b-be40f75723b9" />
link: https://stellar.expert/explorer/testnet/contract/CBNZWGMSJEVG3BFCEAQHQQBTJYKIXJMQXT2AFIVRI5JSWO4XWXPESABV
# 🎉 Event RSVP Smart Contract (Soroban - Stellar)

## 📌 Project Description
This project is a simple decentralized Event RSVP system built using Soroban smart contracts on the Stellar blockchain. It allows users to create events and manage RSVPs in a transparent and tamper-proof way.

## ⚙️ What it does
- Allows a user to create an event with a unique event ID
- Enables other users to RSVP to that event
- Stores RSVPs on-chain using Soroban storage
- Prevents duplicate RSVPs
- Allows retrieval of all attendees for an event

## ✨ Features
- 🪪 **Authentication Required**  
  Only authorized users can create events or RSVP.

- 📅 **Event Creation**  
  Users can create events using a unique identifier.

- ✅ **RSVP System**  
  Users can RSVP to events securely.

- 🚫 **Duplicate Protection**  
  Prevents users from RSVPing multiple times.

- 📊 **Attendee List Retrieval**  
  Fetch all RSVPs for any event.

- ⚡ **Lightweight & Efficient**  
  Uses Soroban's optimized storage and data structures.

## 🔗 Deployed Smart Contract Link: Event RSVP
> _Add your deployed contract link here_  
Example: https://stellar.expert/explorer/testnet/contract/XXXXXXXX

---

## 🛠️ Tech Stack
- Rust (Soroban SDK)
- Stellar Blockchain (Soroban)

## 🚀 How to Use
1. Deploy the contract on Stellar testnet
2. Call `create_event` with:
   - event_id
   - creator address
3. Users call `rsvp` with:
   - event_id
   - their address
4. Call `get_rsvps` to view attendees

---

## 📌 Future Improvements
- Event metadata (name, date, description)
- RSVP limits
- Event cancellation
- NFT-based tickets
- UI frontend integration

---

## 👨‍💻 Author
Your Name - Arnob Mondal
