# Data Model design

This file is for documenting data model hypotheses and the information
learned from their implementations.

This doesn't include the ad-hoc initial attempts.


## Design 1:

### User

Contains:

    * ID
    * Name
    * (Hashed, salted) password
    
### User data

    * Primary key: User ID
    * Array of article IDs
    * Array of user encountered word IDs

### Article

    * ID
    * Title
    * Content (text)
    * Language ID
    * Array of (unique) word IDs
    
### Language

    * Name of language
    * Optional: Definition of extra fields in word (defined in JSON)
    
### Word
    
    * ID
    * Textual representation
    * Text containing JSON with possible extra information

### User encountered word

    * Primary key/foreign key: Word ID
    * Definition
    * Metadata about how well user knows the word
    * Array of article IDs where word was encountered
