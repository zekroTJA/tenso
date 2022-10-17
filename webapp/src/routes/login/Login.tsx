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
    padding: 0 1em;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 1em;
    width: 20em;
  }
`;

export const LoginRoute: React.FC = () => {
  const fetch = useApi();
  const nav = useNavigate();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const login = () => {
    fetch((c) => c.authLogin({ username, password })).then(() => nav("/"));
  };

  return (
    <Conainer>
      <div>
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
        <Button disabled={!username || !password} onClick={login}>
          Login
        </Button>
      </div>
    </Conainer>
  );
};
