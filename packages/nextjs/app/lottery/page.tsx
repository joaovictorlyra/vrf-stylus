"use client";

import { useEffect, useState } from "react";
import type { NextPage } from "next";
import { formatEther } from "viem";
import { useAccount } from "wagmi";
import { Card } from "~~/components/Card";
import { Address } from "~~/components/scaffold-eth";
import { useScaffoldReadContract, useScaffoldWriteContract } from "~~/hooks/scaffold-eth";
import { useTargetNetwork } from "~~/hooks/scaffold-eth/useTargetNetwork";
import { notification } from "~~/utils/scaffold-eth";

const LotteryPage: NextPage = () => {
  const { address: connectedAddress } = useAccount();
  const { targetNetwork } = useTargetNetwork();
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  // Read contract state
  const { data: entryFee } = useScaffoldReadContract({
    contractName: "lottery",
    functionName: "getEntryFee",
  });

  const { data: isOpen } = useScaffoldReadContract({
    contractName: "lottery",
    functionName: "isLotteryOpen",
  });

  const { data: playersCount } = useScaffoldReadContract({
    contractName: "lottery",
    functionName: "getPlayersCount",
  });

  const { data: prizePool } = useScaffoldReadContract({
    contractName: "lottery",
    functionName: "getPrizePool",
  });

  const { data: lastWinner } = useScaffoldReadContract({
    contractName: "lottery",
    functionName: "getLastWinner",
  });

  const { data: owner } = useScaffoldReadContract({
    contractName: "lottery",
    functionName: "getOwner",
  });

  // Write functions
  const { writeContractAsync: enterLottery, isPending: isEntering } = useScaffoldWriteContract("lottery");
  const { writeContractAsync: startDraw, isPending: isStarting } = useScaffoldWriteContract("lottery");

  const handleEnterLottery = async () => {
    if (!entryFee) {
      notification.error("Entry fee not loaded");
      return;
    }

    try {
      await enterLottery({
        functionName: "enter",
        value: entryFee,
      });
      notification.success("Successfully entered the lottery!");
    } catch (error: any) {
      console.error("Error entering lottery:", error);
      notification.error(error.message || "Failed to enter lottery");
    }
  };

  const handleStartDraw = async () => {
    try {
      await startDraw({
        functionName: "draw",
      });
      notification.success("Draw started! Waiting for VRF response...");
    } catch (error: any) {
      console.error("Error starting draw:", error);
      notification.error(error.message || "Failed to start draw");
    }
  };

  const isOwner = connectedAddress && owner ? connectedAddress.toLowerCase() === owner.toLowerCase() : false;

  if (!mounted) {
    return null;
  }

  return (
    <>
      <div className="flex items-center flex-col flex-grow pt-10">
        <div className="px-5 w-full max-w-7xl">
          <h1 className="text-center mb-8">
            <span className="block text-4xl font-bold mb-2">🎰 Chainlink VRF Lottery</span>
            <span className="block text-2xl">Decentralized Random Winner Selection</span>
          </h1>

          {/* What is Chainlink VRF Lottery */}
          <Card className="mb-6">
            <h2 className="text-2xl font-bold mb-4">📚 What is a VRF Lottery?</h2>
            <div className="space-y-3 text-base">
              <p>
                A <strong>Verifiable Random Function (VRF) Lottery</strong> is a transparent and provably fair lottery
                system that uses Chainlink VRF to select winners randomly.
              </p>
              <div className="bg-base-200 p-4 rounded-lg space-y-2">
                <p className="font-semibold">How it works:</p>
                <ol className="list-decimal list-inside space-y-2 ml-2">
                  <li>
                    <strong>Enter:</strong> Players pay an entry fee to join the lottery
                  </li>
                  <li>
                    <strong>Draw:</strong> Owner starts the draw, requesting a random number from Chainlink VRF
                  </li>
                  <li>
                    <strong>Select:</strong> VRF returns verifiable randomness to select the winner
                  </li>
                  <li>
                    <strong>Win:</strong> Winner automatically receives the entire prize pool
                  </li>
                </ol>
              </div>
              <p className="text-sm opacity-70">
                💡 All randomness is cryptographically secure and verifiable on-chain, ensuring complete fairness.
              </p>
            </div>
          </Card>

          {/* Current Lottery Status */}
          <Card className="mb-6">
            <h2 className="text-2xl font-bold mb-4">🎯 Current Lottery Status</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              <div className="bg-base-200 p-4 rounded-lg">
                <p className="text-sm opacity-70 mb-1">Status</p>
                <p className="text-2xl font-bold">{isOpen ? "🟢 Open" : "🔴 Closed"}</p>
              </div>
              <div className="bg-base-200 p-4 rounded-lg">
                <p className="text-sm opacity-70 mb-1">Entry Fee</p>
                <p className="text-2xl font-bold">{entryFee ? `${formatEther(entryFee)} ETH` : "..."}</p>
              </div>
              <div className="bg-base-200 p-4 rounded-lg">
                <p className="text-sm opacity-70 mb-1">Players</p>
                <p className="text-2xl font-bold">{playersCount?.toString() || "0"}</p>
              </div>
              <div className="bg-base-200 p-4 rounded-lg">
                <p className="text-sm opacity-70 mb-1">Prize Pool</p>
                <p className="text-2xl font-bold">{prizePool ? `${formatEther(prizePool)} ETH` : "0 ETH"}</p>
              </div>
              <div className="bg-base-200 p-4 rounded-lg">
                <p className="text-sm opacity-70 mb-1">Network</p>
                <p className="text-xl font-bold">{targetNetwork.name}</p>
              </div>
            </div>
          </Card>

          {/* Enter Lottery */}
          {isOpen && (
            <Card className="mb-6">
              <h2 className="text-2xl font-bold mb-4">🎫 Enter Lottery</h2>
              <p className="mb-4">
                Join the current lottery by paying the entry fee. Winner takes all when the draw is complete!
              </p>
              <div className="flex flex-col sm:flex-row gap-4 items-center">
                <div className="flex-1 bg-base-200 p-4 rounded-lg">
                  <p className="text-sm opacity-70 mb-1">Entry Fee</p>
                  <p className="text-3xl font-bold">{entryFee ? `${formatEther(entryFee)} ETH` : "..."}</p>
                </div>
                <button
                  className="btn btn-primary btn-lg"
                  onClick={handleEnterLottery}
                  disabled={isEntering || !connectedAddress || !isOpen}
                >
                  {isEntering ? <span className="loading loading-spinner"></span> : "🎰 Enter Lottery"}
                </button>
              </div>
            </Card>
          )}

          {/* Owner Controls */}
          {isOwner && (
            <Card className="mb-6 border-2 border-warning">
              <h2 className="text-2xl font-bold mb-4">👑 Owner Controls</h2>
              <div className="space-y-4">
                <div>
                  <p className="mb-2">
                    <strong>Start Draw:</strong> Request random number from Chainlink VRF to select winner
                  </p>
                  <button
                    className="btn btn-accent"
                    onClick={handleStartDraw}
                    disabled={isStarting || !isOpen || !playersCount || playersCount === 0n}
                  >
                    {isStarting ? <span className="loading loading-spinner"></span> : "🎲 Start Draw"}
                  </button>
                  {(!playersCount || playersCount === 0n) && isOpen && (
                    <p className="text-sm text-warning mt-2">⚠️ Need at least 1 player to start draw</p>
                  )}
                </div>
              </div>
            </Card>
          )}

          {/* Latest Winner */}
          {lastWinner && lastWinner !== "0x0000000000000000000000000000000000000000" && (
            <Card className="mb-6 bg-success/10 border-2 border-success">
              <h2 className="text-2xl font-bold mb-4">🏆 Latest Winner</h2>
              <div className="space-y-3">
                <div className="flex justify-between items-center">
                  <span className="font-semibold">Winner:</span>
                  <Address address={lastWinner} />
                </div>
              </div>
            </Card>
          )}

          {/* How to Play */}
          <Card>
            <h2 className="text-2xl font-bold mb-4">🎮 How to Play</h2>
            <div className="space-y-4">
              <div className="flex gap-4">
                <span className="text-3xl">1️⃣</span>
                <div>
                  <h3 className="font-bold text-lg">Connect Your Wallet</h3>
                  <p>Make sure you&apos;re connected to {targetNetwork.name}</p>
                </div>
              </div>
              <div className="flex gap-4">
                <span className="text-3xl">2️⃣</span>
                <div>
                  <h3 className="font-bold text-lg">Get Testnet ETH</h3>
                  <p>
                    Get free testnet tokens from{" "}
                    <a
                      href="https://faucets.chain.link/arbitrum-sepolia"
                      target="_blank"
                      rel="noopener noreferrer"
                      className="link link-primary"
                    >
                      Chainlink Faucet
                    </a>
                  </p>
                </div>
              </div>
              <div className="flex gap-4">
                <span className="text-3xl">3️⃣</span>
                <div>
                  <h3 className="font-bold text-lg">Enter the Lottery</h3>
                  <p>Pay the entry fee to join the current lottery round</p>
                </div>
              </div>
              <div className="flex gap-4">
                <span className="text-3xl">4️⃣</span>
                <div>
                  <h3 className="font-bold text-lg">Wait for Draw</h3>
                  <p>Owner will start the draw when ready, requesting randomness from Chainlink VRF</p>
                </div>
              </div>
              <div className="flex gap-4">
                <span className="text-3xl">5️⃣</span>
                <div>
                  <h3 className="font-bold text-lg">Winner Takes All!</h3>
                  <p>VRF returns random number and winner is automatically selected and paid</p>
                </div>
              </div>
            </div>
          </Card>
        </div>
      </div>
    </>
  );
};

export default LotteryPage;
