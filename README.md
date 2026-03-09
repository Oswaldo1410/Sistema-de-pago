# 💸 PaymentDB - Sistema de Pagos en Solana

## 🚀 ¿Qué construí?
Un backend para un sistema de pagos descentralizado en la blockchain de Solana, construido con **Rust y el framework Anchor**, que registra "recibos" inmutables de cada transacción.

## 💡 Inspiración y Problema
En los sistemas financieros tradicionales, los historiales de pago se guardan en bases de datos centralizadas (como SQL) que pueden ser alteradas o hackeadas. El problema que quería resolver era: **¿Cómo crear un registro de pagos 100% transparente, inmutable y descentralizado?** Decidí usar Solana por su alta velocidad y bajos costos de transacción, ideal para procesar pagos en tiempo real.

## ⚙️ Cómo funciona (Flujo Técnico)
El sistema no usa "tablas" convencionales, sino que utiliza **PDAs (Program Derived Addresses)** para almacenar el estado. El flujo es el siguiente:

1. **Usuario (Wallet):** Inicia y firma una transacción indicando el monto a pagar y un ID de pago único.
2. **Transacción:** Se envía a la red de Solana (Devnet).
3. **Programa (Contrato Inteligente):** Nuestro código en Rust procesa la transferencia real de SOL (Lamports) entre el emisor y el receptor utilizando el `System Program`.
4. **Estado (Base de Datos):** El programa genera una PDA única basada en la llave del emisor y el ID del pago. En esta cuenta PDA se guarda el "recibo" inmutable:
   - Wallet del emisor
   - Wallet del receptor
   - Monto transferido
   - ID del pago
   - Timestamp (Fecha y hora exacta de la red)

## 🧠 Qué aprendí en el proceso
* **El concepto de PDAs:** Entender que en Solana no hay bases de datos relacionales, sino cuentas derivadas criptográficamente que almacenan datos (como nuestro `PaymentRecord`).
* **Desarrollo ágil con Solana Playground:** Cómo escribir, compilar (`build`), generar el Program ID y desplegar (`deploy`) un contrato directamente en la Devnet desde el navegador.
* **Testing en Web3:** Cómo simular transacciones completas (transferencia de fondos + escritura de datos) usando TypeScript y `@coral-xyz/anchor`.

## 🔮 Qué sigue (Próximos pasos)
Si tuviera una semana más para trabajar en este proyecto, implementaría:
1. **Soporte para Tokens (SPL):** Permitir que los pagos se hagan en *USDC* u otras stablecoins, no solo en SOL nativo.
2. **Validaciones de Seguridad:** Añadir límites de monto máximo por transacción o listas blancas (whitelists) de receptores permitidos.
3. **Frontend Web3:** Construir una interfaz gráfica sencilla en React/Next.js para que un usuario final conecte su Phantom Wallet y haga el pago con un clic, sin usar la terminal.

*Program ID en Devnet: `AipAVkxDMjJVMji9jqpUoYDy27JUhJBBatyWY9BsSg2R`*
