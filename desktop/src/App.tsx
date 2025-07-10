import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { onOpenUrl } from "@tauri-apps/plugin-deep-link";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [userInfo, setUserInfo] = useState<any>(null);
  const [isLoggingIn, setIsLoggingIn] = useState(false);

  // Listen for deep link URLs (OAuth callback)
  useEffect(() => {
    const unlistenPromise = onOpenUrl((urls) => {
      console.log("Deep link received:", urls);

      // Handle OAuth callback
      for (const url of urls) {
        if (url.includes("http://localhost:5173/auth/callback")) {
          handleOAuthCallback(url);
          break;
        }
      }
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  const handleOAuthCallback = async (url: string) => {
    try {
      const urlParams = new URL(url);
      const code = urlParams.searchParams.get("code");
      const error = urlParams.searchParams.get("error");

      if (error) {
        console.error("OAuth error:", error);
        setIsLoggingIn(false);
        return;
      }

      if (code) {
        console.log("Authorization code received:", code);

        // In a real app, you would:
        // 1. Send the code to your backend server
        // 2. Exchange it for access tokens
        // 3. Get user info from Google's API

        // For demo purposes, simulate successful login
        setIsLoggedIn(true);
        setUserInfo({
          name: "John Doe",
          email: "john.doe@gmail.com",
          picture: "https://via.placeholder.com/40",
          accessToken: "access_token_from_code_exchange",
        });

        console.log("Login successful!");
      }
    } catch (error) {
      console.error("Error handling OAuth callback:", error);
    } finally {
      setIsLoggingIn(false);
    }
  };

  async function greet() {
    // Learn more about Tauri commands at https://tauri-apps/api/core";
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      {/* Login Section */}
      <div className="auth-section">
        {!isLoggedIn ? (
          <button
            onClick={() => invoke("authenticate")}
            disabled={isLoggingIn}
            className="google-login-btn"
            style={{
              background: isLoggingIn ? "#ccc" : "#4285f4",
              color: "white",
              border: "none",
              padding: "10px 20px",
              borderRadius: "5px",
              cursor: isLoggingIn ? "not-allowed" : "pointer",
              marginBottom: "20px",
              fontSize: "16px",
              opacity: isLoggingIn ? 0.7 : 1,
            }}
          >
            {isLoggingIn ? "� Logging in..." : "�🔐 Login with Google"}
          </button>
        ) : (
          <div className="user-info" style={{ marginBottom: "20px" }}>
            <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
              {userInfo?.picture && (
                <img
                  src={userInfo.picture}
                  alt="Profile"
                  style={{ width: "40px", height: "40px", borderRadius: "50%" }}
                />
              )}
              <div>
                <p style={{ margin: 0 }}>Welcome, {userInfo?.name}!</p>
                <p style={{ margin: 0, fontSize: "12px", color: "#666" }}>
                  {userInfo?.email}
                </p>
              </div>
              <button
                style={{
                  background: "#dc3545",
                  color: "white",
                  border: "none",
                  padding: "5px 10px",
                  borderRadius: "3px",
                  cursor: "pointer",
                  fontSize: "12px",
                }}
              >
                Logout
              </button>
            </div>
          </div>
        )}
      </div>

      {/* <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div> */}
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
