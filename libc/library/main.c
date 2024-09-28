#include <stdio.h>
#include <string.h> 
#include <stdlib.h>
#include "biblioteca.h"

int main (void){
	Libro l[max_num_libri];
	Prestito p[max_num_prestiti];
	int i, dimL, dimP;

	//test esercizio 1
	dimL=leggiLibri("libri.txt", l);
	printf("----- LIBRI -----");
	for(i=0;i<dimL;i++)
		stampaLibro(l[i]);
	dimP=leggiPrestiti("prestiti.txt", p);
	printf("\n----- PRESTITI -----");
	for(i=0;i<dimP;i++)
		stampaPrestito(p[i]);
 
    // test esercizio 3
    ordina(l, dimL);
	printf("\n----- LIBRI (ORDINATI) -----");
	for(i=0;i<dimL;i++)
		stampaLibro(l[i]);

	printf("\n");
	return 0;
}
