import { useEffect, useState } from "react";
import {
  BASE_FEE,
  Networks,
  Operation,
  TransactionBuilder,
  rpc,
  scValToNative,
  xdr,
} from "@stellar/stellar-sdk";

import {
  getNetwork,
  isConnected,
  requestAccess,
  signTransaction,
} from "@stellar/freighter-api";

import "./App.css";

const CONTRACT_ID =
  "CDFYU6UXJ4OL7SQQZYSBAMVHQQZJM43NJKNYIISQCJN63T5EBGLVWVR2";

const RPC_URL = "https://soroban-testnet.stellar.org";
const NETWORK_PASSPHRASE = Networks.TESTNET;

const server = new rpc.Server(RPC_URL);

function convertirError(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }

  return String(error);
}

function convertirASymbol(valor: string): xdr.ScVal {
  return xdr.ScVal.scvSymbol(valor);
}

function obtenerValoresResultado(retval: xdr.ScVal): string[] {
  const resultado = scValToNative(retval);

  if (Array.isArray(resultado)) {
    return resultado.map((valor) => String(valor));
  }

  return [String(resultado)];
}

export default function App() {
  const [address, setAddress] = useState("");
  const [nombre, setNombre] = useState("Stellar");
  const [resultado, setResultado] = useState<string[]>([]);
  const [estado, setEstado] = useState("");
  const [transactionHash, setTransactionHash] = useState("");

  /*
   * Intenta recuperar una wallet que ya haya autorizado la dApp.
   */
  useEffect(() => {
    async function recuperarWallet() {
      try {
        const connection = await isConnected();

        if (connection.error) {
          console.error(connection.error);
          return;
        }

        if (!connection.isConnected) {
          return;
        }

        const access = await requestAccess();

        if (access.error) {
          console.error(access.error);
          return;
        }

        if (access.address) {
          setAddress(access.address);
          setEstado("Wallet conectada");
        }
      } catch (error) {
        console.error("No se pudo recuperar la wallet:", error);
      }
    }

    recuperarWallet();
  }, []);

  async function conectarWallet() {
    try {
      setEstado("Comprobando Freighter...");

      const connection = await isConnected();

      if (connection.error) {
        throw new Error(connection.error.message);
      }

      if (!connection.isConnected) {
        throw new Error(
          "Freighter no está instalado o no está disponible en este navegador",
        );
      }

      /*
       * Solicita permiso para que esta dApp acceda a la dirección pública.
       */
      const access = await requestAccess();

      if (access.error) {
        throw new Error(access.error.message);
      }

      if (!access.address) {
        throw new Error(
          "No se obtuvo una dirección. Autoriza esta dApp en Freighter.",
        );
      }

      /*
       * Verifica que Freighter esté utilizando Testnet.
       */
      const network = await getNetwork();

      if (network.error) {
        throw new Error(network.error.message);
      }

      if (network.network !== "TESTNET") {
        throw new Error(
          `Cambia Freighter a Testnet. Red actual: ${network.network}`,
        );
      }

      setAddress(access.address);
      setEstado("Wallet conectada en Testnet");
    } catch (error) {
      setEstado(`Error al conectar: ${convertirError(error)}`);
    }
  }

  function crearOperacionHello() {
    return Operation.invokeContractFunction({
      contract: CONTRACT_ID,
      function: "hello",
      args: [convertirASymbol(nombre)],
    });
  }

  async function simularHello() {
    try {
      setEstado("Preparando simulación...");
      setResultado([]);
      setTransactionHash("");

      if (!address) {
        throw new Error("Conecta primero una wallet de Testnet");
      }

      if (!nombre.trim()) {
        throw new Error("Escribe un nombre");
      }

      const sourceAccount = await server.getAccount(address);

      const transaction = new TransactionBuilder(sourceAccount, {
        fee: BASE_FEE,
        networkPassphrase: NETWORK_PASSPHRASE,
      })
        .addOperation(crearOperacionHello())
        .setTimeout(30)
        .build();

      const simulation = await server.simulateTransaction(transaction);

      if (!rpc.Api.isSimulationSuccess(simulation)) {
        throw new Error(
          `La simulación falló: ${JSON.stringify(simulation)}`,
        );
      }

      const valores = obtenerValoresResultado(
        simulation.result.retval,
      );

      setResultado(valores);
      setEstado(
        "Simulación correcta. No se publicó ninguna transacción.",
      );
    } catch (error) {
      setEstado(`Error en la simulación: ${convertirError(error)}`);
    }
  }

  async function enviarHello() {
    try {
      setEstado("Preparando transacción...");
      setResultado([]);
      setTransactionHash("");

      if (!address) {
        throw new Error("Conecta primero una wallet de Testnet");
      }

      if (!nombre.trim()) {
        throw new Error("Escribe un nombre");
      }

      const sourceAccount = await server.getAccount(address);

      const transaction = new TransactionBuilder(sourceAccount, {
        fee: BASE_FEE,
        networkPassphrase: NETWORK_PASSPHRASE,
      })
        .addOperation(crearOperacionHello())
        .setTimeout(30)
        .build();

      /*
       * La simulación calcula los recursos necesarios para Soroban.
       */
      setEstado("Simulando transacción...");

      const simulation = await server.simulateTransaction(transaction);

      if (!rpc.Api.isSimulationSuccess(simulation)) {
        throw new Error(
          `La simulación falló: ${JSON.stringify(simulation)}`,
        );
      }

      const assembledTransaction = rpc
        .assembleTransaction(transaction, simulation)
        .build();

      setEstado("Esperando firma en Freighter...");

      const signed = await signTransaction(
        assembledTransaction.toXDR(),
        {
          networkPassphrase: NETWORK_PASSPHRASE,
        },
      );

      if (signed.error) {
        throw new Error(signed.error.message);
      }

      if (!signed.signedTxXdr) {
        throw new Error("Freighter no devolvió una transacción firmada");
      }

      const signedTransaction = TransactionBuilder.fromXDR(
        signed.signedTxXdr,
        NETWORK_PASSPHRASE,
      );

      setEstado("Enviando transacción a Testnet...");

      const sendResponse = await server.sendTransaction(
        signedTransaction,
      );

      if (sendResponse.status === "ERROR") {
        throw new Error(JSON.stringify(sendResponse));
      }

      const hash = sendResponse.hash;

      setTransactionHash(hash);
      setEstado("Transacción enviada. Esperando confirmación...");

      let confirmada = false;

      while (!confirmada) {
        await new Promise((resolve) => {
          setTimeout(resolve, 2000);
        });

        const transactionResponse = await server.getTransaction(hash);

        if (
          transactionResponse.status ===
          rpc.Api.GetTransactionStatus.SUCCESS
        ) {
          confirmada = true;
          setEstado("Transacción confirmada en Testnet");

          if (transactionResponse.result) {
            const valores = obtenerValoresResultado(
              transactionResponse.result,
            );

            setResultado(valores);
          }
        }

        if (
          transactionResponse.status ===
          rpc.Api.GetTransactionStatus.FAILED
        ) {
          throw new Error(
            "La transacción fue rechazada por la red",
          );
        }
      }
    } catch (error) {
      setEstado(`Error al enviar: ${convertirError(error)}`);
    }
  }

  const botonesDeshabilitados =
    !address || !nombre.trim();

  return (
    <main className="container">
      <section className="card">
        <p className="eyebrow">
          Soroban · Stellar Testnet
        </p>

        <h1>Hello Contract</h1>

        <p className="description">
          Invoca la función <code>hello(nombre)</code> del contrato.
        </p>

        <div className="contract">
          <span>Contrato</span>

          <code>{CONTRACT_ID}</code>
        </div>

        {!address ? (
          <button onClick={conectarWallet}>
            Conectar Freighter
          </button>
        ) : (
          <div className="wallet">
            <span>Wallet conectada</span>

            <code>{address}</code>
          </div>
        )}

        <label htmlFor="nombre">
          Nombre
        </label>

        <input
          id="nombre"
          type="text"
          value={nombre}
          maxLength={32}
          placeholder="Escribe un nombre"
          onChange={(event) => {
            setNombre(event.target.value);
          }}
        />

        <div className="actions">
          <button
            className="secondary"
            onClick={simularHello}
            disabled={botonesDeshabilitados}
          >
            Simular llamada
          </button>

          <button
            onClick={enviarHello}
            disabled={botonesDeshabilitados}
          >
            Firmar y enviar
          </button>
        </div>

        {estado && (
          <p className="status">
            {estado}
          </p>
        )}

        {resultado.length > 0 && (
          <div className="result">
            <h2>Resultado</h2>

            <div className="result-values">
              {resultado.map((valor, indice) => (
                <span key={`${valor}-${indice}`}>
                  {valor}
                </span>
              ))}
            </div>
          </div>
        )}

        {transactionHash && (
          <p className="hash">
            Hash de la transacción:{" "}

            <a
              href={`https://stellar.expert/explorer/testnet/tx/${transactionHash}`}
              target="_blank"
              rel="noreferrer"
            >
              {transactionHash}
            </a>
          </p>
        )}
      </section>
    </main>
  );
}

