/* only for 8 bit systems, that means only S0, S1, S9 markers */

/* no checksum tested... */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define NO 1
#define YES 0

#define IS_NOT_HEX(a) ( ((a<'0') || (a>'9')) && ((a<'A')||(a>'F')) )
#define MAKE_BIN(a) if (a<='9') a-='0';else a-=('A'-10);


int main(int argc, char *argv[])
{
 FILE *in;
 FILE *out;
 unsigned char memory[65536];
 unsigned int size=8192;
 unsigned char fill=0;
 char file_in[256];
 char file_out[256];
 char fchar;
 int overwrite=NO;
 int i=0;
 int read_counter;
 int entries_found=0;
 int first_line=0;
 long out_counter;
 file_in[0]=0;
 file_out[0]=0;
 if (argc>1)
 {
  while (argv[i])
  {
        char *pointer=argv[i];
        if ((pointer[0]=='-')||(pointer[0]=='/')||(pointer[0]==':')||(pointer[0]=='\\'))
        {
         if ((pointer[1]=='h')||(pointer[1]=='H')||(pointer[1]=='?'))
         {
          printf("-h   Help\n");
          printf("-H   Help\n");
          printf("-?   Help\n");
          printf("-ifile_in input filename\n");
          printf("-ofile_out output filename\n");
          printf("-m#number size of output file\n");
          printf("-f#number fill overhead with this byte\n");
          printf("-w do overwrite output (default no overwrite)\n");
          printf("Checksum information is not validated!\n");
          printf("\nWritten 1996 by Christopher Salomon\n");

          return 0;
         }
         if ((pointer[1]=='w')||(pointer[1]=='W'))
         {
          overwrite=YES;
         }
         if ((pointer[1]=='i')||(pointer[1]=='I'))
         {
          strcpy(file_in,pointer+2);
         }
         if ((pointer[1]=='o')||(pointer[1]=='O'))
         {
          strcpy(file_out,pointer+2);
         }
         if ((pointer[1]=='m')||(pointer[1]=='M'))
         {
          size = atoi(pointer+2);
         }
         if ((pointer[1]=='f')||(pointer[1]=='F'))
         {
          fill = (unsigned char) atoi(pointer+2);
         }
        }
        i++;
  }
 }
 if (file_in[0]==0)
 {
  printf("no input file given...\n");
  return 1;
 }
 if (file_out[0]==0)
 {
  printf("no output file given, using \"out.bin\"...\n");
  strcpy(file_out,"out.bin");
 }
 if ((in = fopen(file_in, "rb"))== NULL)
 {
  printf("cannot open input file...\n");
  return 2;
 }
 if (overwrite==NO)
 {
  if ((out = fopen(file_out, "rb"))!= NULL)
  {
        fclose(out);
        fclose(in);
        printf("output file exists, option \"-w\" to overwrite...\n");
        return 3;
  }
 }
 if ((out = fopen(file_out, "wb"))==NULL)
 {
  fclose(in);
  printf("can't open output file...\n");
  return 4;
 }
 printf("-----------------\n");
 printf("input file.......: %s\n",file_in);
 printf("output file......: %s\n",file_out);
 printf("binary size......: %u\n",size);
 printf("fill byte........: %i\n",fill);
 for (out_counter=0;out_counter<65536;out_counter++)
 {
  memory[out_counter]=fill;
 }
 out_counter=0;
 do
 {
  read_counter=fread(&fchar,sizeof(char),1,in);
 }
 while ((read_counter)&&(fchar!='S'));
 while (read_counter)
 {
  unsigned int position=0;
  int how_many=0;
  fread(&fchar,sizeof(char),1,in);
  if (first_line==0)
  {
   first_line=1;
   if (fchar=='0') /* header marker */
   {
    do
    {
     read_counter=fread(&fchar,sizeof(char),1,in);
    }
    while ((read_counter)&&(fchar!='S'));
    continue;
   }
  }
  if (fchar=='9') /* end marker */
  {
        break;
  }
  if (fchar!='1') /* data marker */
  {
        fclose(in);
        fclose(out);
        printf("illegal entry encountered, aborting...\n");
        return 5;
  }
  fread(&fchar,sizeof(char),1,in);
  if (IS_NOT_HEX(fchar))
  {
        printf("hex expected (but not found 1a), aborting...\n");
        fclose(in);
        fclose(out);
        return 6;
  }
  MAKE_BIN(fchar)
  how_many+=fchar*16;
  fread(&fchar,sizeof(char),1,in);
  if (IS_NOT_HEX(fchar))
  {
        printf("hex expected (but not found 1b), aborting...\n");
        fclose(in);
        fclose(out);
        return 6;
  }
  MAKE_BIN(fchar)
  how_many+=fchar;
  how_many-=3;
  fread(&fchar,sizeof(char),1,in);
  if (IS_NOT_HEX(fchar))
  {
        printf("hex expected (but not found 2a), aborting...\n");
        fclose(in);
        fclose(out);
        return 6;
  }
  MAKE_BIN(fchar)
  position+=fchar*16*16*16;
  fread(&fchar,sizeof(char),1,in);
  if (IS_NOT_HEX(fchar))
  {
        printf("hex expected (but not found 2b), aborting...\n");
        fclose(in);
        fclose(out);
        return 6;
  }
  MAKE_BIN(fchar)
  position+=fchar*16*16;
  fread(&fchar,sizeof(char),1,in);
  if (IS_NOT_HEX(fchar))
  {
        printf("hex expected (but not found 2c), aborting...\n");
        fclose(in);
        fclose(out);
        return 6;
  }
  MAKE_BIN(fchar)
  position+=fchar*16;
  fread(&fchar,sizeof(char),1,in);
  if (IS_NOT_HEX(fchar))
  {
        printf("hex expected (but not found 2d), aborting...\n");
        fclose(in);
        fclose(out);
        return 6;
  }
  MAKE_BIN(fchar)
  position+=fchar;
  if (position>size)
  {
        printf("warning, postion higher than size...\n");
  }
  while (how_many)
  {
        unsigned char poke_value=0;
        how_many--;
        fread(&fchar,sizeof(char),1,in);
        if (IS_NOT_HEX(fchar))
        {
         printf("hex expected (but not found 3a), aborting...\n");
         fclose(in);
         fclose(out);
         return 6;
        }
        MAKE_BIN(fchar)
        poke_value+=fchar*16;
        fread(&fchar,sizeof(char),1,in);
        if (IS_NOT_HEX(fchar))
        {
         printf("hex expected (but not found 3b), aborting...\n");
         fclose(in);
         fclose(out);
         return 6;
        }
        MAKE_BIN(fchar)
        poke_value+=fchar;
        memory[position]=poke_value;
        position++;
        out_counter++;
  }
  fread(&fchar,sizeof(char),1,in);
  fread(&fchar,sizeof(char),1,in);
  entries_found++;
  do
  {
   read_counter=fread(&fchar,sizeof(char),1,in);
  }
  while ((read_counter)&&(fchar!='S'));
 }
 fwrite(memory,sizeof(char),size,out);
 fclose(in);
 fclose(out);
 printf("bytes processed..: %i\n",out_counter);
 printf("S1 entries found.: %i\n",entries_found);
 printf("-----------------\n");

 printf("ok!\n");
 return 0;
}
