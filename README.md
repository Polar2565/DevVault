# DevVault

## 🇬🇧 English

DevVault is a Solana program built with **Rust and Anchor** that allows users to create a personal on-chain vault and manage entries using a simple CRUD system.

The program demonstrates core Solana development concepts such as **Program Derived Addresses (PDA)**, account management, and interaction through blockchain transactions.

### Features

- Create a personal vault (PDA)
- Register entries inside the vault
- Toggle entry state (active / inactive)
- Remove entries
- Store data on-chain

### Technologies

- Rust
- Anchor Framework
- Solana Devnet
- Solana Playground
- TypeScript (for testing)

### How it works

1. A user creates a **vault account** using a PDA derived from their wallet.
2. The vault stores entries on-chain.
3. Users can:
   - add entries
   - update entry state
   - remove entries
4. All interactions occur through **Solana transactions**.

### Program Instructions

- `iniciar_vault` → Creates the user's vault account.
- `registrar_entry` → Adds a new entry to the vault.
- `alternar_estado` → Toggles the active state of an entry.
- `remover_entry` → Removes an entry from the vault.
- `ver_registros` → Displays stored entries.

---

## 🇪🇸 Español

DevVault es un programa desarrollado en **Solana utilizando Rust y el framework Anchor** que permite a los usuarios crear una bóveda personal en la blockchain y administrar registros mediante un sistema CRUD.

Este proyecto demuestra conceptos fundamentales del desarrollo en Solana como **Program Derived Addresses (PDA)**, manejo de cuentas y la interacción con programas mediante transacciones en la blockchain.

### Características

- Crear una bóveda personal (PDA)
- Registrar entradas dentro de la bóveda
- Cambiar el estado de una entrada (activo / inactivo)
- Eliminar entradas
- Almacenar datos en la blockchain

### Tecnologías

- Rust
- Anchor Framework
- Solana Devnet
- Solana Playground
- TypeScript (para pruebas)

### Cómo funciona

1. Un usuario crea una **cuenta vault** utilizando un PDA derivado de su wallet.
2. La bóveda almacena registros en la blockchain.
3. Los usuarios pueden:
   - agregar entradas
   - actualizar el estado de una entrada
   - eliminar entradas
4. Todas las interacciones ocurren mediante **transacciones en Solana**.

### Instrucciones del programa

- `iniciar_vault` → Crea la cuenta de vault del usuario.
- `registrar_entry` → Agrega una nueva entrada a la bóveda.
- `alternar_estado` → Cambia el estado activo de una entrada.
- `remover_entry` → Elimina una entrada.
- `ver_registros` → Muestra los registros almacenados.

---
## Author / Autor

Javier Solis Martinez
