import React from 'react';
import './index.css';

const StatCard = ({ label, value }) => (
  <div className="flex flex-col items-center justify-center p-6 glass-panel transform transition hover:-translate-y-2 hover:shadow-[0_0_30px_rgba(102,252,241,0.3)]">
    <div className="text-4xl font-bold text-accent-cyan mb-2">{value}</div>
    <div className="text-sm tracking-widest text-slate-400 uppercase">{label}</div>
  </div>
);

const FeatureCard = ({ title, desc, icon }) => (
  <div className="glass-panel p-8 text-left transition-all hover:border-accent-cyan hover:shadow-[0_0_20px_rgba(102,252,241,0.2)]">
    <div className="text-3xl mb-4">{icon}</div>
    <h3 className="text-2xl font-bold mb-3 text-white">{title}</h3>
    <p className="text-slate-400 leading-relaxed">{desc}</p>
  </div>
);

function App() {
  return (
    <div className="min-h-screen bg-dark-900 overflow-hidden relative selection:bg-accent-cyan selection:text-dark-900">

      {/* Background Orbs */}
      <div className="absolute top-0 left-0 w-full h-full overflow-hidden pointer-events-none z-0">
        <div className="absolute top-[-10%] left-[-10%] w-96 h-96 bg-accent-blue rounded-full mix-blend-multiply filter blur-[128px] opacity-40 animate-blob"></div>
        <div className="absolute top-[-10%] right-[-10%] w-96 h-96 bg-accent-purple rounded-full mix-blend-multiply filter blur-[128px] opacity-40 animate-blob animation-delay-2000"></div>
        <div className="absolute bottom-[-20%] left-[20%] w-[40rem] h-[40rem] bg-accent-cyan rounded-full mix-blend-multiply filter blur-[128px] opacity-20 animate-blob animation-delay-4000"></div>
      </div>

      <div className="relative z-10 mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-20">

        {/* Navbar / Header area */}
        <nav className="flex items-center justify-between py-6 mb-16 animate-fade-in-up">
          <div className="flex items-center gap-3 text-2xl font-black tracking-tighter text-white">
            <img src="/awsm.svg" alt="AWSM Logo" className="w-10 h-10" />
            <div>AWSM<span className="text-accent-cyan">.WASM</span></div>
          </div>
          <div className="flex gap-6 text-sm font-semibold tracking-wide">
            <a href="#features" className="hover:text-accent-cyan transition-colors">FEATURES</a>
            <a href="#docs" className="hover:text-accent-cyan transition-colors">DOCUMENTATION</a>
            <a href="https://github.com/hoytnix/awsm.wasm" className="hover:text-accent-cyan transition-colors">GITHUB</a>
          </div>
        </nav>

        {/* Hero Section */}
        <main className="text-center mt-20 mb-32 animate-fade-in-up" style={{ animationDelay: '0.2s' }}>
          <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full border border-accent-blue/30 bg-accent-blue/10 text-accent-cyan text-sm tracking-widest uppercase mb-8 shadow-[0_0_20px_rgba(69,162,158,0.2)]">
            <span className="relative flex h-3 w-3">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-accent-cyan opacity-75"></span>
              <span className="relative inline-flex rounded-full h-3 w-3 bg-accent-cyan"></span>
            </span>
            v1.0 Active Standard
          </div>
          <h1 className="text-6xl md:text-8xl font-black mb-8 leading-tight tracking-tight">
            Deterministic AST <br />
            <span className="gradient-text italic pr-2">Mutations.</span>
          </h1>
          <p className="max-w-3xl mx-auto text-xl text-slate-400 mb-12 font-light leading-relaxed">
            A total, sub-Turing, purely functional DSL for <strong className="text-white">mathematical determinism</strong>.
            Replacing stochastic code generation with secure, verifiable WebAssembly pipelines via Stable Pointer Hashes (SPH) and Patchiest Wire Schema (PWS) v1.1.
          </p>
          <div className="flex items-center justify-center gap-6">
            <button className="btn-primary group">
              Get Started
              <span className="ml-2 inline-block transition-transform group-hover:translate-x-1">→</span>
            </button>
            <button className="px-8 py-3 rounded-full font-semibold border border-slate-700 hover:border-accent-cyan hover:text-accent-cyan transition-all">
              View Specs
            </button>
          </div>
        </main>

        {/* Stats Grid */}
        <section className="grid grid-cols-2 md:grid-cols-4 gap-6 mb-32 animate-fade-in-up" style={{ animationDelay: '0.4s' }}>
          <StatCard value="1:1" label="Semantic Parity" />
          <StatCard value="256-bit" label="Collision Resistance" />
          <StatCard value="O(1)" label="State Determinism" />
          <StatCard value="100%" label="Sub-Turing Pure" />
        </section>

        {/* Features & Architecture */}
        <section id="features" className="mb-32">
          <div className="text-center mb-16">
            <h2 className="text-4xl font-bold mb-4">Architected for <span className="text-accent-cyan">Purity</span></h2>
            <p className="text-slate-400 max-w-2xl mx-auto">AWSM executes as a total function inside wasm32-wasi secure sandboxes.</p>
          </div>

          <div className="grid md:grid-cols-3 gap-8">
            <FeatureCard
              icon="⚡"
              title="Mathematical Determinism"
              desc="Identical Pristine State Hash + identical AWSM bytecode → identical PWS receipts across all conformant runtimes."
            />
            <FeatureCard
              icon="🔒"
              title="Secure Sandbox"
              desc="Executes strictly within the KILN Sovereign Sanctuary using the wasm32-wasi ABI. No time, no randomness, no unbounded IO."
            />
            <FeatureCard
              icon="🧬"
              title="Stable Pointer Hashes"
              desc="Powered by parallel BLAKE3. AST nodes are strictly normalized, ensuring structural integrity across large monoliths."
            />
          </div>
        </section>

        {/* Terminal/Code snippet Section */}
        <section className="glass-panel overflow-hidden mb-32 relative group">
          <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-accent-blue to-accent-purple"></div>
          <div className="flex border-b border-white/10 px-4 py-3 bg-dark-800/50">
            <div className="flex gap-2">
              <div className="w-3 h-3 rounded-full bg-red-500"></div>
              <div className="w-3 h-3 rounded-full bg-yellow-500"></div>
              <div className="w-3 h-3 rounded-full bg-green-500"></div>
            </div>
            <div className="mx-auto text-xs text-slate-500 font-mono">PWS v1.1 Envelope</div>
          </div>
          <div className="p-8 bg-[#0d1117] overflow-x-auto text-sm font-mono leading-relaxed">
            <pre className="text-slate-300">
              <span className="text-accent-cyan">{`{`}</span>
              <span className="text-accent-blue">"v"</span>: <span className="text-accent-purple">1</span>,
              <span className="text-accent-blue">"tx_id"</span>: <span className="text-green-400">"&lt;sha256&gt;"</span>,
              <span className="text-accent-blue">"purity_hash"</span>: <span className="text-green-400">"&lt;blake3-final-state&gt;"</span>,
              <span className="text-accent-blue">"actions"</span>: [
              <span className="text-accent-cyan">{`{`}</span>
              <span className="text-accent-blue">"type"</span>: <span className="text-yellow-300">"MUTATE_CALL"</span>,
              <span className="text-accent-blue">"target_urn"</span>: <span className="text-green-400">"urn:awsm:src/api.rs:route_fn/a1b2c3d4"</span>,
              <span className="text-accent-blue">"payload"</span>: <span className="text-accent-cyan">{`{`}</span>
              <span className="text-accent-blue">"rename"</span>: <span className="text-green-400">"handle_request_safely"</span>,
              <span className="text-accent-blue">"inject_args"</span>: <span className="text-accent-cyan">{`{`}</span> <span className="text-accent-blue">"timeout"</span>: <span className="text-green-400">"30"</span> <span className="text-accent-cyan">{`}`}</span>
              <span className="text-accent-cyan">{`}`}</span>
              <span className="text-accent-cyan">{`}`}</span>
              ]
              <span className="text-accent-cyan">{`}`}</span>
            </pre>
          </div>
        </section>

        {/* Footer */}
        <footer className="border-t border-white/10 pt-12 pb-8 flex flex-col md:flex-row justify-between items-center text-sm text-slate-500">
          <p>© 2026 Alchememe / Athanor Team. Functional code mutations.</p>
          <div className="flex gap-4 mt-4 md:mt-0">
            <a href="#" className="hover:text-accent-cyan transition-colors">RFC v1.0</a>
            <a href="#" className="hover:text-accent-cyan transition-colors">Terms</a>
            <a href="#" className="hover:text-accent-cyan transition-colors">Privacy</a>
          </div>
        </footer>
      </div>
    </div>
  );
}

export default App;
