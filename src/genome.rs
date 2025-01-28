/*
 * struct file for the bacdive metanalyzer
 * */

#[derive(Debug, Clone, PartialEq, PartialOrd)]
 pub   struct Bacterialphyla {
    pub    domain: String,
    pub   phylum: String,
    pub     class: String,
    pub      order: String,
    pub     family: String,
    pub     genus: String,
    pub    species: String,
    pub    names: String,
    }

#[derive(Debug, Clone, PartialOrd, PartialEq)]
   pub  struct GenomeReferences {
       pub  referencesection: String,
       pub  referencelink: String,
    }
