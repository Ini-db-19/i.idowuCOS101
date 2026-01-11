--
-- PostgreSQL database dump
--

\restrict ebKBXwlfIcBwlf7pAXDzAiAh6xRxypzgzVdqIt3szkQ7zZum6KsouZDfdey920D

-- Dumped from database version 16.11
-- Dumped by pg_dump version 16.11

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: departments; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.departments (
    dept_managerid integer NOT NULL,
    dno character(1) NOT NULL,
    dname text NOT NULL,
    dlocation text NOT NULL,
    pno integer NOT NULL
);


ALTER TABLE public.departments OWNER TO postgres;

--
-- Name: employees; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.employees (
    eid integer NOT NULL,
    ename text NOT NULL,
    dno character(1) NOT NULL,
    esal integer NOT NULL,
    age integer NOT NULL,
    phone character(11) NOT NULL
);


ALTER TABLE public.employees OWNER TO postgres;

--
-- Name: projects; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.projects (
    pno integer NOT NULL,
    pname character(1) NOT NULL,
    pduration text NOT NULL,
    project_managerid integer NOT NULL
);


ALTER TABLE public.projects OWNER TO postgres;

--
-- Data for Name: departments; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.departments (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	Administration	Ikeja	44
101	2	Account	        Egbeda	11
100	3	Packaging	    Ajah	44
120	4	Research	    V.I	    33
97	5	Account	        Magodo	22
122	6	Operations	    Mile 2	44
107	7	Packaging	    Ketu	55
\.


--
-- Data for Name: employees; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.employees (eid, ename, dno, esal, age, phone) FROM stdin;
101	Alade Joy	    2	250000	33	08023089832
100	Mustapha Ali	3	175000	32	08063285831
107	Alokwe Martin	7	380000	48	07090082812
97	Dankade Aminat	5	550000	40	09023688832
108	Josiah Joshua	1	120000	30	08053189131
102	Mankinde Mary	2	450000	55	09023487830
120	Adeleke Jane	4	200000	38	07061045862
122	Osahon Mark	    6	320000	44	08022289842
117	Suleman Ajayi	3	800000	50	07030089811
104	Kuti Lawal	    1	750000	35	09145689842
\.


--
-- Data for Name: projects; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.projects (pno, pname, pduration, project_managerid) FROM stdin;
11	A	9 Months	102
22	B	14 Months	97
33	C	16 Months	120
44	D	25 Months	108
55	E	9 Months	107
\.


--
-- Name: departments departments_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.departments
    ADD CONSTRAINT departments_pkey PRIMARY KEY (dept_managerid);


--
-- Name: employees employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.employees
    ADD CONSTRAINT employees_pkey PRIMARY KEY (eid);


--
-- Name: projects projects_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_pkey PRIMARY KEY (project_managerid);


--
-- PostgreSQL database dump complete
--

\unrestrict ebKBXwlfIcBwlf7pAXDzAiAh6xRxypzgzVdqIt3szkQ7zZum6KsouZDfdey920D

