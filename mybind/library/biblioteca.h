#include<stdlib.h>
#include<stdio.h>
#include<string.h>

#define dimCodice 5
#define dimTitolo 41
#define dimAutore 31
#define dimCliente 21

#define max_num_libri 30
#define max_num_prestiti 20

#ifndef LIBRO
#define LIBRO
typedef struct{
	char codice[dimCodice];
	char titolo[dimTitolo];
	char autore [dimAutore];
	char genere;
	int anno;
	int copie;
} Libro;
#endif

#ifndef PRESTITO
#define PRESTITO
typedef struct{
	char codice[dimCodice];
	char cliente[dimCliente];
	int giorno;
	int mese;
	int anno;
} Prestito;
#endif

int leggiLibri (char* nomefile, Libro libri[]);
void stampaLibro (Libro l);
int leggiPrestiti (char* nomefile, Prestito prestiti[]);
void stampaPrestito (Prestito p);
int calcolaRitardi(Prestito p[], int dimP); 
int inserisciPrestito (FILE * fp, Libro l[], int dimL, Prestito p[], int dimP);
void ordina (Libro libri[], int dimL);
