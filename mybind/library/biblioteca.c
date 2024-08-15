#include "biblioteca.h"

int leggiLibri (char* nomefile, Libro l[]){
	int i;
	FILE * fp;

	if((fp=fopen(nomefile,"r"))==NULL)
		exit(1);
	i = 0;
	while ((fscanf(fp,"%s %s %s %c %d %d\n",
				l[i].codice, l[i].titolo, l[i].autore,
			       	&(l[i].genere),	&(l[i].anno), &(l[i].copie)) == 6) && (i < max_num_libri)){
        i++;
    }
	fclose(fp);
	return i;
}

void stampaLibro (Libro l){
	printf("\nLibro - Codice: %s\t Titolo: %s\t Autore: %s\t Genere: %c\t Anno: %d\t Copie: %d", l.codice, l.titolo, l.autore, l.genere, l.anno, l.copie);
}

int leggiPrestiti (char* nomefile, Prestito p[]){
	int i;
	FILE * fp;

    i = 0;
	if((fp=fopen(nomefile,"r"))==NULL)
		exit(1);

	while ((fscanf(fp,"%s %s %d %d %d\n",
				p[i].codice, p[i].cliente,
			       	&(p[i].giorno),	&(p[i].mese), &(p[i].anno)) == 5) && (i < max_num_prestiti)){
        i++;
    }
	fclose(fp);
	return i;
}

void stampaPrestito (Prestito p){
	printf("\nPrestito - Codice: %s\t Cliente: %s\t Giorno Scadenza: %d\t Mese Scadenza: %d\t Anno Scadenza: %d", p.codice, p.cliente, p.giorno, p.mese, p.anno);
}

void scambia(Libro* a, Libro* b){
    Libro temp;
    temp = *a;
    *a = *b;
    *b = temp;
}


void ordina (Libro libri[], int dimL){
    // bubble sort
    int i;
    int ordinato = 0;
    while (dimL > 1 && ordinato == 0){
        ordinato = 1;
        for(i = 0; i < dimL - 1; i++){
            if(libri[i].anno > libri[i+1].anno){
                scambia(&libri[i], &libri[i+1]);
                ordinato = 0;
            }
        }
        dimL--;
    }
}
