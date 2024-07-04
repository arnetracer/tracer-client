pub const TARGETS: &[&str] = &[
    "uptime",
    "java -Xmx250m -Dfastqc.output_dir=. -XX:ParallelGCThreads=1 -Djava.awt.headless",
    "STAR",
    "bowtie2",
    "bwa",
    "salmon",
    "hisat2",
    "HOMER",
    "samtools",
    "bedtools",
    "deeptools",
    "macs3",
    "plotCoverage",
    "MACS33",
    "Genrich",
    "TopHat",
    "JAMM",
    "fastqc",
    "multiqc",
    "fastp",
    "PEAR",
    "Trimmomatic",
    "sra-toolkit",
    "Picard",
    "cutadapt",
    "cellranger",
    "STATsolo",
    "scTE",
    "scanpy",
    "Seurat",
    "LIGER",
    "SC3",
    "Louvain",
    "Leiden",
    "Garnett",
    "Monocle",
    "Harmony",
    "PAGA",
    "Palantir",
    "velocity",
    "CellPhoneDB",
    "CellChat",
    "NicheNet",
    "FIt-SNE",
    "umap",
    "bbmap",
    "cuffdiff",
    "RNA-SeQC",
    "RSeQC",
    "Trimgalore",
    "UCHIME",
    "Erange",
    "X-Mate",
    "SpliceSeq",
    "casper",
    "DESeq",
    "EdgeR",
    "Kallisto",
    "pairtools",
    "HiCExplorer",
    "GITAR",
    "TADbit",
    "Juicer",
    "HiC-Pro",
    "cooler",
    "cooltools",
    "runHiC",
    "HTSlib",
    "zlib",
    "libbz2",
    "liblzma",
    "libcurl",
    "libdeflate",
    "ncurses",
    "pthread",
];
