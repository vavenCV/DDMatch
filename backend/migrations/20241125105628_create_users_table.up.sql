-- Add migration script here

CREATE TABLE dragodinde (
  	id 			    		BIGINT UNSIGNED PRIMARY KEY NOT NULL,
	name 		    		TEXT NOT NULL,
    description     		TEXT NOT NULL,
	genre_id				BIGINT UNSIGNED NOT NULL,
	couleur_finale_id		BIGINT UNSIGNED NOT NULL,
	parent_pere_id			BIGINT UNSIGNED,
	parent_mere_id			BIGINT UNSIGNED,
	gestation_nb			SMALLINT UNSIGNED NOT NULL,
	capacite_nb				SMALLINT UNSIGNED NOT NULL,
	capacite_1_id			BIGINT UNSIGNED,
	capacite_2_id			BIGINT UNSIGNED,
	capacite_3_id			BIGINT UNSIGNED,
	capacite_4_id			BIGINT UNSIGNED,
  proba_couleur     TEXT NOT NULL
);

CREATE TABLE genre (
  	id						BIGINT UNSIGNED PRIMARY KEY NOT NULL,
	name 		    		TEXT NOT NULL
);

CREATE TABLE capacite (
  	id						BIGINT UNSIGNED PRIMARY KEY NOT NULL,
	name 		    		TEXT NOT NULL,
    description     		TEXT NOT NULL
);

CREATE TABLE couleur_finale (
  	id 			    		BIGINT UNSIGNED PRIMARY KEY NOT NULL,
	name 		    		TEXT NOT NULL,
	generation_id			BIGINT UNSIGNED NOT NULL,
	couleur_nb				INT UNSIGNED NOT NULL,
	couleur_1_id			BIGINT UNSIGNED NOT NULL,
	couleur_2_id			BIGINT UNSIGNED,
	pods_base				BIGINT UNSIGNED NOT NULL,
	pods_par_level			INT UNSIGNED NOT NULL,
	energie_base			INT UNSIGNED NOT NULL,
	energie_par_level		INT UNSIGNED NOT NULL,
	maturite				BIGINT UNSIGNED NOT NULL,
	gestation_h				BIGINT UNSIGNED NOT NULL,
	coef_pourcent			INT UNSIGNED NOT NULL
);

CREATE TABLE couleur (
  	id 			    		BIGINT UNSIGNED PRIMARY KEY NOT NULL,
	name 		    		TEXT NOT NULL
);

CREATE TABLE generation (
	id 			    		BIGINT UNSIGNED PRIMARY KEY NOT NULL,
	gen_number				INT UNSIGNED NOT NULL
);

insert into genre (id, name)
values
  (1, "Male"),
  (2, "Femelle")

insert into generation (id, gen_number)
values
  (1, 1),
  (2, 2),
  (3, 3),
  (4, 4),
  (5, 5),
  (6, 6),
  (7, 7),
  (8, 8),
  (9, 9),
  (10, 10)


insert into couleur (id, name)
values
  (1, "Amande"),
  (2, "Dorée"),
  (3, "Ebène"),
  (4, "Emeraude"),
  (5, "Indigo"),
  (6, "Ivoire"),
  (7, "Orchidée"),
  (8, "Pourpre"),
  (9, "Prune"),
  (10, "Rousse"),
  (11, "Turquoise")

insert into couleur_finale (id, name, generation_id, couleur_nb, couleur_1_id, couleur_2_id, pods_base, 
pods_par_level, energie_base, energie_par_level, maturite, gestation_h, coef_pourcent)
values	
  (1, 	"Rousse", 				1, 	1, 	10, 0, 	100, 5, 	1000, 10, 1000, 	48, 100),
  (2, 	"Amande", 				1, 	1, 	1, 	0, 	100, 5, 	1000, 10, 1000, 	48, 100),
  (3, 	"Dorée", 				1, 	1, 	2, 	0, 	100, 5, 	1000, 10, 10000, 	48, 20),
  (4, 	"Rousse - Amande", 		2, 	2, 	10, 1, 	150, 5, 	1100, 15, 2000, 	60, 80),
  (5, 	"Rousse - Dorée", 		2, 	2, 	10, 2, 	150, 5, 	1100, 15, 2000, 	60, 80),
  (6, 	"Amande - Dorée", 		2, 	2, 	1, 	2, 	150, 5, 	1100, 15, 2000, 	60, 80),
  (7, 	"Indigo", 				3, 	1, 	5, 	0, 	200, 10, 	1200, 20, 3000, 	75, 80),
  (8, 	"Ebène", 				3, 	1, 	3, 	0, 	200, 10, 	1200, 20, 3000, 	75, 80),
  (9, 	"Rousse - Indigo", 		4, 	2, 	10, 5, 	250, 10, 	1300, 25, 4000, 	84, 80),
  (10, 	"Rousse - Ebène", 		4, 	2, 	10, 3, 	250, 10, 	1300, 25, 4000, 	84, 80),
  (11, 	"Amande - Indigo", 		4, 	2, 	1, 	5, 	250, 10, 	1300, 25, 4000, 	84, 80),
  (12, 	"Amande - Ebène", 		4, 	2, 	1, 	3, 	250, 10, 	1300, 25, 4000, 	84, 80),
  (13, 	"Dorée - Indigo", 		4, 	2, 	2, 	5, 	250, 10, 	1300, 25, 4000, 	84, 80),
  (14, 	"Dorée - Ebène", 		4, 	2, 	2, 	3, 	250, 10, 	1300, 25, 4000, 	84, 80),
  (15, 	"Indigo - Ebène", 		4, 	2, 	5, 	3, 	250, 10, 	1300, 25, 4000, 	84, 80),
  (16, 	"Pourpre", 				5, 	1, 	8, 	0, 	300, 15, 	1400, 30, 5000, 	96, 60),
  (17, 	"Orchidée", 			5, 	1, 	7, 	0, 	300, 15, 	1400, 30, 5000, 	96, 60),
  (18, 	"Rousse - Pourpre", 	6, 	2, 	10, 8, 	350, 15, 	1500, 35, 6000, 	108, 60),
  (19, 	"Rousse - Orchidée", 	6, 	2, 	10, 7, 	350, 15, 	1500, 35, 6000, 	108, 60),
  (20, 	"Amande - Pourpre", 	6, 	2, 	1, 	8, 	350, 15, 	1500, 35, 6000, 	108, 60),
  (21, 	"Amande - Orchidée", 	6, 	2, 	1, 	7, 	350, 15, 	1500, 35, 6000, 	108, 60),
  (22, 	"Dorée - Pourpre", 		6, 	2, 	2, 	8, 	350, 15, 	1500, 35, 6000, 	108, 60),
  (23, 	"Dorée - Orchidée", 	6, 	2, 	2, 	7, 	350, 15, 	1500, 35, 6000, 	108, 60),
  (24, 	"Indigo - Pourpre", 	6, 	2, 	5, 	8, 	350, 15, 	1500, 35, 6000, 	108, 60),
  (25, 	"Indigo - Orchidée", 	6, 	2, 	5, 	7, 	350, 15, 	1500, 35, 6000, 	108, 60),
  (26, 	"Ebène - Pourpre", 		6, 	2, 	3, 	8, 	350, 15, 	1500, 35, 6000, 	108, 60),
  (27, 	"Ebène - Orchidée", 	6, 	2, 	3, 	7, 	350, 15, 	1500, 35, 6000, 	108, 60),
  (28, 	"Pourpre - Orchidée", 	6, 	2, 	8, 	7, 	350, 15, 	1500, 35, 6000, 	108, 60),
  (29, 	"Ivoire", 				7, 	1, 	6, 	0, 	400, 20, 	1600, 40, 7000, 	120, 60),
  (30, 	"Turquoise", 			7, 	1, 	11, 0, 	400, 20, 	1600, 40, 7000, 	120, 60),
  (31, 	"Rousse - Ivoire", 		8, 	2, 	10, 6, 	450, 20, 	1700, 45, 8000, 	132, 40),
  (32, 	"Rousse - Turquoise", 	8, 	2, 	10, 11, 450, 20, 	1700, 45, 8000, 	132, 40),
  (33, 	"Amande - Ivoire", 		8, 	2, 	1, 	6, 	450, 20, 	1700, 45, 8000, 	132, 40),
  (34, 	"Amande - Turquoise", 	8, 	2, 	1, 	11, 450, 20, 	1700, 45, 8000, 	132, 40),
  (35, 	"Dorée - Ivoire", 		8, 	2, 	2, 	6, 	450, 20, 	1700, 45, 8000, 	132, 40),
  (36, 	"Dorée - Turquoise", 	8, 	2, 	2, 	11, 450, 20, 	1700, 45, 8000, 	132, 40),
  (37, 	"Indigo - Ivoire", 		8, 	2, 	5, 	6, 	450, 20, 	1700, 45, 8000, 	132, 40),
  (38, 	"Indigo - Turquoise", 	8, 	2, 	5, 	11, 450, 20, 	1700, 45, 8000, 	132, 40),
  (39, 	"Ebène - Ivoire", 		8, 	2, 	3, 	6, 	450, 20, 	1700, 45, 8000, 	132, 40),
  (40, 	"Ebène - Turquoise", 	8, 	2, 	3, 	11, 450, 20, 	1700, 45, 8000, 	132, 40),
  (41, 	"Pourpre - Ivoire", 	8, 	2, 	8, 	6, 	450, 20, 	1700, 45, 8000, 	132, 40),
  (42, 	"Pourpre - Turquoise", 	8, 	2, 	8, 	11, 450, 20, 	1700, 45, 8000, 	132, 40),
  (43, 	"Orchidée - Ivoire", 	8, 	2, 	7, 	6, 	450, 20, 	1700, 45, 8000, 	132, 40),
  (44, 	"Orchidée - Turquoise", 8, 	2, 	7, 	11, 450, 20, 	1700, 45, 8000, 	132, 40),
  (45, 	"Ivoire - Turquoise", 	8, 	2, 	6, 	11, 450, 20, 	1700, 45, 8000, 	132, 40),
  (46, 	"Emeraude", 			9, 	1, 	4, 	0, 	500, 25, 	1800, 50, 9000, 	144, 40),	
  (47, 	"Prune", 				9, 	1, 	9, 	0, 	500, 25, 	1800, 50, 9000, 	144, 40),
  (48, 	"Rousse - Emeraude", 	10, 2, 	10, 4, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (49, 	"Rousse - Prune", 		10, 2, 	10, 9, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (50, 	"Amande - Emeraude", 	10, 2, 	1, 	4, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (51, 	"Amande - Prune", 		10, 2, 	1, 	9, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (52, 	"Dorée - Emeraude", 	10, 2, 	2, 	4, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (53, 	"Dorée - Prune", 		10, 2, 	2, 	9, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (54, 	"Indigo - Emeraude", 	10, 2, 	5, 	4, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (55, 	"Indigo - Prune", 		10, 2, 	5, 	9, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (56, 	"Ebène - Emeraude", 	10, 2, 	3, 	4, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (57, 	"Ebène - Prune", 		10, 2, 	3, 	9, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (58, 	"Pourpre - Emeraude", 	10, 2, 	8, 	4, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (59, 	"Pourpre - Prune", 		10, 2, 	8, 	9, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (60, 	"Orchidée - Emeraude", 	10, 2, 	7, 	4, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (61, 	"Orchidée - Prune", 	10, 2, 	7, 	9, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (62, 	"Ivoire - Emeraude", 	10, 2, 	6, 	4, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (63, 	"Ivoire - Prune", 		10, 2, 	6, 	9, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (64, 	"Turquoise - Emeraude", 10, 2, 	11, 4, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (65, 	"Turquoise - Prune", 	10, 2, 	11, 9, 	550, 25, 	1900, 55, 10000, 	156, 20),
  (66, 	"Emeraude - Prune", 	10, 2, 	4, 	9, 	550, 25, 	1900, 55, 10000, 	156, 20)

insert into capacite (id, name, description)
values
  (1, "Précoce", "Double la vitesse d'obtention des points de maturité."),
  (2, "Amoureuse", "Double la vitesse d'obtention des points en amour."),
  (3, "Endurante", "Double la vitesse d'obtention des points d'endurance."),
  (4, "Sage", "Double les points d'expérience cédées à la monture."),
  (5, "Reproducteur", "Augmente la portée de 1 individus (seulement chez les femelles)."),
  (6, "Porteur", "Double le nombre de pods pouvant être porter par la dragodinde."),
  (7, "Infatigable", "Doubles la vitesse d'obtention des points d'énergie."),
  (8, "Prédisposition génétique", "Augmente l'influence génétique de la dinde lors d'un croisement.")