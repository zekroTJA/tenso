import { Button } from "../../components/Button";
import { Input } from "../../components/Input";
import styled from "styled-components";
import { useApi } from "../../hooks/useApi";
import { useNavigate } from "react-router-dom";
import { useState } from "react";

const Conainer = styled.div`
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;

  > div {
    > p {
      text-align: center;
      margin-bottom: 2em;
      max-width: 30em;
    }

    padding: 0 1em;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 1em;
    width: 20em;
  }
`;

export const InitRoute: React.FC = () => {
  const fetch = useApi();
  const nav = useNavigate();
  const [token, setToken] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const init = () => {
    fetch((c) => c.authInit({ token, username, password })).then(() =>
      nav("/login")
    );
  };

  return (
    <Conainer>
      <div>
        <p>
          This instance it not initialized.
          <br />
          <br />
          Please enter the initialization token which you can find in the logs
          of the backend. Also enter a username and password you want to use as
          login credentials.
        </p>
        <Input
          type="password"
          placeholder="Initialization Token"
          value={token}
          onInput={(e) => setToken(e.currentTarget.value)}
        />
        <Input
          placeholder="Username"
          value={username}
          onInput={(e) => setUsername(e.currentTarget.value)}
        />
        <Input
          type="password"
          placeholder="Password"
          value={password}
          onInput={(e) => setPassword(e.currentTarget.value)}
        />
        <Button disabled={!token || !username || !password} onClick={init}>
          Initialize
        </Button>
      </div>
    </Conainer>
  );
};
