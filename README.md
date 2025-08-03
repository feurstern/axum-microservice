# axum-microservice


<table>
  <tr>
    <td>
     <img width="200" height="657" alt="image" src="https://github.com/user-attachments/assets/5863afc7-191a-47f8-9d5b-c65286faad75" />
    </td>
    <td>
    <img width="750" height="427" alt="image" src="https://github.com/user-attachments/assets/d49c4726-71a4-491f-bcf0-237835f47b53" />
    </td>
  </tr>
</table>




## ✨ Overviews

| Area | Highlights |
|------|------------|
| Features | Login, Todo |
| ORM | DIESEL POSTGRES |
| Build | Pure Rust → `wasm-bindgen` → tiny JS wrapper |
| Dev server | `src/main/rs` (≈ 80 LOC) – no Node required |

---



## 🚀 Quick start

    # once per machine
    rustup target add wasm32-unknown-unknown
    cargo install wasm-pack

    #Diesel 
    cargo install diesel
    

    # development (watch mode)
    wasm-pack build --target web --out-dir pkg --dev --watch &
    cargo run       # → http://localhost:2121

### Production build

    wasm-pack build

