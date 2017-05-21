--
-- PostgreSQL database dump
--

-- Dumped from database version 9.6.3
-- Dumped by pg_dump version 9.6.3

-- Started on 2017-05-21 10:21:36

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 2150 (class 1262 OID 16394)
-- Name: myapp; Type: DATABASE; Schema: -; Owner: docker
--

\connect docker

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 1 (class 3079 OID 12387)
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- TOC entry 2152 (class 0 OID 0)
-- Dependencies: 1
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


--
-- TOC entry 2 (class 3079 OID 16471)
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- TOC entry 2153 (class 0 OID 0)
-- Dependencies: 2
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


SET search_path = public, pg_catalog;

--
-- TOC entry 206 (class 1255 OID 16450)
-- Name: createnewauthid(character varying); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION createnewauthid(user_email character varying DEFAULT 'NOT_GIVEN'::character varying) RETURNS uuid
    LANGUAGE plpgsql LEAKPROOF
    AS $$DECLARE new_key UUID;
BEGIN
	INSERT INTO user_auth_keys
	VALUES(uuid_generate_v4(), user_email, current_timestamp, true)
	RETURNING auth_key into new_key;
    RETURN(new_key);
END
$$;


ALTER FUNCTION public.createnewauthid(user_email character varying) OWNER TO postgres;

--
-- TOC entry 207 (class 1255 OID 16457)
-- Name: createnewsequence(character varying, bigint, uuid); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION createnewsequence(input_id character varying, starting_seed bigint, user_key uuid) RETURNS bigint
    LANGUAGE plpgsql LEAKPROOF
    AS $$DECLARE new_sequence BIGINT;
BEGIN
    INSERT INTO sequences(sequence_id, sequence_value, last_update, api_key)
    VALUES (input_id, starting_seed, current_timestamp, user_key)
	RETURNING sequence_value INTO new_sequence;
    
    RETURN(new_sequence);
END
    
$$;


ALTER FUNCTION public.createnewsequence(input_id character varying, starting_seed bigint, user_key uuid) OWNER TO postgres;

--
-- TOC entry 214 (class 1255 OID 16482)
-- Name: incrementandgetsequencenumber(character varying, uuid); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION incrementandgetsequencenumber(input_id character varying, auth_id uuid) RETURNS bigint
    LANGUAGE plpgsql STRICT LEAKPROOF
    AS $$
  DECLARE updated_sequence BIGINT;
  BEGIN
  	UPDATE sequences
  	SET sequence_value = sequence_value + 1,
      last_update = current_timestamp
  	WHERE sequence_id = input_id
    AND api_key = auth_id
  	RETURNING sequence_value INTO updated_sequence;
  	RETURN(updated_sequence);
 END
$$;


ALTER FUNCTION public.incrementandgetsequencenumber(input_id character varying, auth_id uuid) OWNER TO postgres;

--
-- TOC entry 205 (class 1255 OID 16444)
-- Name: loguse(uuid, character varying); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION loguse(api_key uuid, sequence_id character varying) RETURNS void
    LANGUAGE sql STRICT LEAKPROOF
    AS $$

	INSERT INTO usage_log
    VALUES(api_key, current_timestamp, sequence_id); 

$$;


ALTER FUNCTION public.loguse(api_key uuid, sequence_id character varying) OWNER TO postgres;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- TOC entry 188 (class 1259 OID 16461)
-- Name: sequences; Type: TABLE; Schema: public; Owner: docker
--

CREATE TABLE sequences (
    sequence_id character varying(255) NOT NULL,
    sequence_value bigint,
    last_update date,
    api_key uuid NOT NULL
);


ALTER TABLE sequences OWNER TO "docker";

--
-- TOC entry 187 (class 1259 OID 16414)
-- Name: usage_log; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE usage_log (
    auth_id uuid,
    "timestamp" timestamp with time zone,
    sequence_id character varying(255)
);


ALTER TABLE usage_log OWNER TO postgres;

--
-- TOC entry 186 (class 1259 OID 16406)
-- Name: user_auth_keys; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE user_auth_keys (
    auth_key uuid NOT NULL,
    email_addr character varying(8000) NOT NULL,
    create_date timestamp with time zone NOT NULL,
    active boolean NOT NULL
);


ALTER TABLE user_auth_keys OWNER TO postgres;

--
-- TOC entry 2026 (class 2606 OID 16465)
-- Name: sequences sequences_pkey; Type: CONSTRAINT; Schema: public; Owner: docker
--

ALTER TABLE ONLY sequences
    ADD CONSTRAINT sequences_pkey PRIMARY KEY (sequence_id, api_key);


--
-- TOC entry 2024 (class 2606 OID 16413)
-- Name: user_auth_keys user_auth_keys_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY user_auth_keys
    ADD CONSTRAINT user_auth_keys_pkey PRIMARY KEY (auth_key);


--
-- TOC entry 2028 (class 2606 OID 16466)
-- Name: sequences FK_auth_id; Type: FK CONSTRAINT; Schema: public; Owner: docker
--

ALTER TABLE ONLY sequences
    ADD CONSTRAINT "FK_auth_id" FOREIGN KEY (api_key) REFERENCES user_auth_keys(auth_key) ON DELETE CASCADE;


--
-- TOC entry 2027 (class 2606 OID 16422)
-- Name: usage_log FK_users_auth; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY usage_log
    ADD CONSTRAINT "FK_users_auth" FOREIGN KEY (auth_id) REFERENCES user_auth_keys(auth_key);


-- Completed on 2017-05-21 10:21:37

--
-- PostgreSQL database dump complete
--

