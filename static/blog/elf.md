---
title: What is an ELF?
author: Pascal Behmenburg
date: 2025-08-03
---
# What is an ELF? { #what-is-an-elf }

~written on August 03, 2025 by Pascal Behmenburg~

---

## The ELF Header { #the-elf-header }

Did you ever ask yourself how the OS knows that something is an executable file? I have. That is when I came to find out about the Executable and Linkable Format (ELF).


The original definition of the ELF header is:

```c
#define EI_NIDENT 16

typedef struct {
    unsigned char e_ident[EI_NIDENT];
    Elf32_Half e_type;
    Elf32_Half e_machine;
    Elf32_Word e_version;
    Elf32_Addr e_entry;
    Elf32_Off e_phoff;
    Elf32_Off e_shoff;
    Elf32_Word e_flags;
    Elf32_Half e_ehsize;
    Elf32_Half e_phentsize;
    Elf32_Half e_phnum;
    Elf32_Half e_shentsize;
    Elf32_Half e_shnum;
    Elf32_Half e_shstrndx;
} Elf32_Ehdr;
```

It is the start of every executable file. It contains information about the file format, such as the type of file, the architecture it was compiled for, and the entry point of the program.

You can vaguely split it up in a few sections:

- **Identification**: The first 16 bytes of the file, which contain information about the file format, such as the type of file, the architecture it was compiled for, and the entry point of the program.
- **Program Header Table**: The program header table is a table of program headers, which contain information about the segments of the file, such as the loadable segments, the dynamic linking information, and the relocation information.
- **Section Header Table**: The section header table is a table of section headers, which contain information about the sections of the file, such as the text section, the data section, and the symbol table.

### Program Header Table { #program-header-table }

The program header table is a table of program headers, which contain information about the segments of the file, such as the loadable segments, the dynamic linking information, and the relocation information.

### Section Header Table { #section-header-table }

The section header table is a table of section headers, which contain information about the sections of the file, such as the text section, the data section, and the symbol table.

## Exploration { #exploration }

To explore these concepts further and understand how this looks like in practice, I wrote [this](https://github.com/pascalbehmenburg/bin-exploration/blob/main/elf.py) python script, which interprets some aspects of the elf header and section header table, which caught my interest.

If you also want to explore the details of the elf header, I recommend reading the [Tool Interface Standard (TIS) Portable Formats Specification](https://refspecs.linuxfoundation.org/elf/TIS1.1.pdf), which I have used to implement my partial python implementation. You can also take a look at the elf header somewhere on your system, if you are running some Linux distro.

When executing the python implementation on an example bin, we can see the following output:

<details>
    <summary>Show output</summary>

```python
ElfHeader(
        ElfIdentifier(
                magic=b'\x7fELF',
                file_class=ElfFileClass.ELFCLASS64,
                data_encoding=ElfDataEncoding.ELFDATA2LSB,
                version=ElfVersion.EV_CURRENT,
        ),
        e_type=ElfFileType.ET_DYN,
        e_machine=ElfMachine.EM_AMD_X86_64,
        e_version=ElfVersion.EV_CURRENT,
        e_entry=0x1040,
        e_phoff=0x40,
        e_shoff=0x34b8,
        e_flags=0x0,
        e_ehsize=0x40,
        e_phentsize=0x38,
        e_phnum=0xe,
        e_shentsize=0x40,
        e_shnum=0x1e,
        e_shstrndx=0x1d,
)
[ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x0,
                name='',
        ),
        sh_type=SectionHeaderType.SHT_NULL,
        sh_flags=SectionHeaderFlags.None,
        sh_addr=0x0,
        sh_offset=0x0,
        sh_size=0x0,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x0,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x1b,
                name='.note.gnu.property',
        ),
        sh_type=SectionHeaderType.SHT_NOTE,
        sh_flags=SectionHeaderFlags.SHF_ALLOC,
        sh_addr=0x350,
        sh_offset=0x350,
        sh_size=0x40,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x8,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x2e,
                name='.note.gnu.build-id',
        ),
        sh_type=SectionHeaderType.SHT_NOTE,
        sh_flags=SectionHeaderFlags.SHF_ALLOC,
        sh_addr=0x390,
        sh_offset=0x390,
        sh_size=0x24,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x4,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x41,
                name='.interp',
        ),
        sh_type=SectionHeaderType.SHT_PROGBITS,
        sh_flags=SectionHeaderFlags.SHF_ALLOC,
        sh_addr=0x3b4,
        sh_offset=0x3b4,
        sh_size=0x1c,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x1,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x49,
                name='.gnu.hash',
        ),
        sh_type=0x6ffffff6,
        sh_flags=SectionHeaderFlags.SHF_ALLOC,
        sh_addr=0x3d0,
        sh_offset=0x3d0,
        sh_size=0x1c,
        sh_link=0x5,
        sh_info=0x0,
        sh_addralign=0x8,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x53,
                name='.dynsym',
        ),
        sh_type=SectionHeaderType.SHT_DYNSYM,
        sh_flags=SectionHeaderFlags.SHF_ALLOC,
        sh_addr=0x3f0,
        sh_offset=0x3f0,
        sh_size=0xa8,
        sh_link=0x6,
        sh_info=0x1,
        sh_addralign=0x8,
        sh_entsize=0x18,
), ElfSectionStringTable(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x5b,
                name='.dynstr',
        ),
        sh_type=SectionHeaderType.SHT_STRTAB,
        sh_flags=SectionHeaderFlags.SHF_ALLOC,
        sh_addr=0x498,
        sh_offset=0x498,
        sh_size=0x8d,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x1,
        sh_entsize=0x0,
        strings=['', 'puts', '__libc_start_main', '__cxa_finalize', 'libc.so.6', 'GLIBC_2.2.5', 'GLIBC_2.34', '_ITM_deregisterTMCloneTable', '__gmon_start__', '_ITM_registerTMCloneTable'],
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x63,
                name='.gnu.version',
        ),
        sh_type=0x6fffffff,
        sh_flags=SectionHeaderFlags.SHF_ALLOC,
        sh_addr=0x526,
        sh_offset=0x526,
        sh_size=0xe,
        sh_link=0x5,
        sh_info=0x0,
        sh_addralign=0x2,
        sh_entsize=0x2,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x70,
                name='.gnu.version_r',
        ),
        sh_type=0x6ffffffe,
        sh_flags=SectionHeaderFlags.SHF_ALLOC,
        sh_addr=0x538,
        sh_offset=0x538,
        sh_size=0x30,
        sh_link=0x6,
        sh_info=0x1,
        sh_addralign=0x8,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x7f,
                name='.rela.dyn',
        ),
        sh_type=SectionHeaderType.SHT_RELA,
        sh_flags=SectionHeaderFlags.SHF_ALLOC,
        sh_addr=0x568,
        sh_offset=0x568,
        sh_size=0xc0,
        sh_link=0x5,
        sh_info=0x0,
        sh_addralign=0x8,
        sh_entsize=0x18,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x89,
                name='.rela.plt',
        ),
        sh_type=SectionHeaderType.SHT_RELA,
        sh_flags=SectionHeaderFlags.SHF_ALLOC|64,
        sh_addr=0x628,
        sh_offset=0x628,
        sh_size=0x18,
        sh_link=0x5,
        sh_info=0x17,
        sh_addralign=0x8,
        sh_entsize=0x18,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x93,
                name='.init',
        ),
        sh_type=SectionHeaderType.SHT_PROGBITS,
        sh_flags=SectionHeaderFlags.SHF_ALLOC|SHF_EXECINSTR,
        sh_addr=0x1000,
        sh_offset=0x1000,
        sh_size=0x1b,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x4,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x8e,
                name='.plt',
        ),
        sh_type=SectionHeaderType.SHT_PROGBITS,
        sh_flags=SectionHeaderFlags.SHF_ALLOC|SHF_EXECINSTR,
        sh_addr=0x1020,
        sh_offset=0x1020,
        sh_size=0x20,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x10,
        sh_entsize=0x10,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x99,
                name='.text',
        ),
        sh_type=SectionHeaderType.SHT_PROGBITS,
        sh_flags=SectionHeaderFlags.SHF_ALLOC|SHF_EXECINSTR,
        sh_addr=0x1040,
        sh_offset=0x1040,
        sh_size=0x113,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x10,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x9f,
                name='.fini',
        ),
        sh_type=SectionHeaderType.SHT_PROGBITS,
        sh_flags=SectionHeaderFlags.SHF_ALLOC|SHF_EXECINSTR,
        sh_addr=0x1154,
        sh_offset=0x1154,
        sh_size=0xd,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x4,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0xa5,
                name='.rodata',
        ),
        sh_type=SectionHeaderType.SHT_PROGBITS,
        sh_flags=SectionHeaderFlags.SHF_ALLOC,
        sh_addr=0x2000,
        sh_offset=0x2000,
        sh_size=0x10,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x4,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0xad,
                name='.eh_frame_hdr',
        ),
        sh_type=SectionHeaderType.SHT_PROGBITS,
        sh_flags=SectionHeaderFlags.SHF_ALLOC,
        sh_addr=0x2010,
        sh_offset=0x2010,
        sh_size=0x24,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x4,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0xbb,
                name='.eh_frame',
        ),
        sh_type=SectionHeaderType.SHT_PROGBITS,
        sh_flags=SectionHeaderFlags.SHF_ALLOC,
        sh_addr=0x2038,
        sh_offset=0x2038,
        sh_size=0x7c,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x8,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0xc5,
                name='.note.ABI-tag',
        ),
        sh_type=SectionHeaderType.SHT_NOTE,
        sh_flags=SectionHeaderFlags.SHF_ALLOC,
        sh_addr=0x20b4,
        sh_offset=0x20b4,
        sh_size=0x20,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x4,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0xd3,
                name='.init_array',
        ),
        sh_type=0xe,
        sh_flags=SectionHeaderFlags.SHF_WRITE|SHF_ALLOC,
        sh_addr=0x3dd0,
        sh_offset=0x2dd0,
        sh_size=0x8,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x8,
        sh_entsize=0x8,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0xdf,
                name='.fini_array',
        ),
        sh_type=0xf,
        sh_flags=SectionHeaderFlags.SHF_WRITE|SHF_ALLOC,
        sh_addr=0x3dd8,
        sh_offset=0x2dd8,
        sh_size=0x8,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x8,
        sh_entsize=0x8,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0xeb,
                name='.dynamic',
        ),
        sh_type=SectionHeaderType.SHT_DYNAMIC,
        sh_flags=SectionHeaderFlags.SHF_WRITE|SHF_ALLOC,
        sh_addr=0x3de0,
        sh_offset=0x2de0,
        sh_size=0x1e0,
        sh_link=0x6,
        sh_info=0x0,
        sh_addralign=0x8,
        sh_entsize=0x10,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0xf4,
                name='.got',
        ),
        sh_type=SectionHeaderType.SHT_PROGBITS,
        sh_flags=SectionHeaderFlags.SHF_WRITE|SHF_ALLOC,
        sh_addr=0x3fc0,
        sh_offset=0x2fc0,
        sh_size=0x28,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x8,
        sh_entsize=0x8,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0xf9,
                name='.got.plt',
        ),
        sh_type=SectionHeaderType.SHT_PROGBITS,
        sh_flags=SectionHeaderFlags.SHF_WRITE|SHF_ALLOC,
        sh_addr=0x3fe8,
        sh_offset=0x2fe8,
        sh_size=0x20,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x8,
        sh_entsize=0x8,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x102,
                name='.data',
        ),
        sh_type=SectionHeaderType.SHT_PROGBITS,
        sh_flags=SectionHeaderFlags.SHF_WRITE|SHF_ALLOC,
        sh_addr=0x4008,
        sh_offset=0x3008,
        sh_size=0x10,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x8,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x108,
                name='.bss',
        ),
        sh_type=SectionHeaderType.SHT_NOBITS,
        sh_flags=SectionHeaderFlags.SHF_WRITE|SHF_ALLOC,
        sh_addr=0x4018,
        sh_offset=0x3018,
        sh_size=0x8,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x1,
        sh_entsize=0x0,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x10d,
                name='.comment',
        ),
        sh_type=SectionHeaderType.SHT_PROGBITS,
        sh_flags=SectionHeaderFlags.None,
        sh_addr=0x0,
        sh_offset=0x3018,
        sh_size=0x1b,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x1,
        sh_entsize=0x1,
), ElfSection(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x1,
                name='.symtab',
        ),
        sh_type=SectionHeaderType.SHT_SYMTAB,
        sh_flags=SectionHeaderFlags.None,
        sh_addr=0x0,
        sh_offset=0x3038,
        sh_size=0x240,
        sh_link=0x1c,
        sh_info=0x6,
        sh_addralign=0x8,
        sh_entsize=0x18,
), ElfSectionStringTable(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x9,
                name='.strtab',
        ),
        sh_type=SectionHeaderType.SHT_STRTAB,
        sh_flags=SectionHeaderFlags.None,
        sh_addr=0x0,
        sh_offset=0x3278,
        sh_size=0x129,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x1,
        sh_entsize=0x0,
        strings=['', 'example.c', '_DYNAMIC', '__GNU_EH_FRAME_HDR', '_GLOBAL_OFFSET_TABLE_', '__libc_start_main@GLIBC_2.34', '_ITM_deregisterTMCloneTable', 'puts@GLIBC_2.2.5', '_edata', '_fini', '__data_start', '__gmon_start__', '__dso_handle', '_IO_stdin_used', '_end', '__bss_start', 'main', '__TMC_END__', '_ITM_registerTMCloneTable', '__cxa_finalize@GLIBC_2.2.5', '_init'],
), ElfSectionStringTable(
        endianness=ElfDataEncoding.ELFDATA2LSB,
        sh_name=SectionHeaderName(
                offset=0x11,
                name='.shstrtab',
        ),
        sh_type=SectionHeaderType.SHT_STRTAB,
        sh_flags=SectionHeaderFlags.None,
        sh_addr=0x0,
        sh_offset=0x33a1,
        sh_size=0x116,
        sh_link=0x0,
        sh_info=0x0,
        sh_addralign=0x1,
        sh_entsize=0x0,
        strings=['', '.symtab', '.strtab', '.shstrtab', '.note.gnu.property', '.note.gnu.build-id', '.interp', '.gnu.hash', '.dynsym', '.dynstr', '.gnu.version', '.gnu.version_r', '.rela.dyn', '.rela.plt', '.init', '.text', '.fini', '.rodata', '.eh_frame_hdr', '.eh_frame', '.note.ABI-tag', '.init_array', '.fini_array', '.dynamic', '.got', '.got.plt', '.data', '.bss', '.comment'],
)]
```

</details>
