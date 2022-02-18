MADSKILLZ (I'm sorry)






                                  a points to inner box on heap, memory of that inner box is leaked                                                          
                                  if no one uses Box::from_raw or other explicit mem free technique                                                          
                                let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));                                                        
                                    |                                            |       |      \                                                            
                                    |                                            |       |       \                                                           
                        stack       ----------------------------------|     -    |       |        \                                                          
                                                                      |          |       |         \                                                         
                             +-----------------------+                |          |       |          \                                                        
                               Box returned by outer |                |          |       |           \                                                      -
                               Box::new(), passed    |                |          |       |            \                                                      
                               throught stack     --------------------|-----------       |             \                                                     
                               points to another     |                |                  |              \                                                    
                               Box on heap           |                |                  |               \              -                                    
                             +-----------------------+                |                  /                \                                                  
                                         |                            |                 /                  \    -                                            
                                        -|                            |                /                    \                                                
                                         |Box on stack points         |               /                      \                                               
                       -----------------------------------------------|--------------/------------------------\-------------------                           
                                         |                            |             /                          \                                             
                                         |                      +-----|-----------------+         +-----------------------+                                  
                                         |                      |                       |         |                       |                                  
                                         ------------------------    inner Box          |         |      that mem would   |                                  
                        heap               to Box on heap       |    that mem leaked    |         |         be freed      |                                  
                                                                |                       |----------                       |                                  
                                                                |                       | points  |                       |                                  
                                           -                    +-----------------------+  to cb  +-----------------------+                                  
                                                                                          -                                                                  
                                                                                                                                                             
                                                                                                                -                                            
                                                                  THAT MEM IS LEAKED due to Box::into_raw                                                    
                                                                  see https://doc.rust-lang.org/std/boxed/struct.Box.html#examples-23                        
                                                                                                                                                             
                                                                                                                                                             
                                                                                                                                                             
                                                                                                                                                             
                                                                                                                                                             
                                                                                                                                                             
                                                                                                                                                             
                                                                                                                                                             
                                                                                                                                                             
                                                                                                                             -                               