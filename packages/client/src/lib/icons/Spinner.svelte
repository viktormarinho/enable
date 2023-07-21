<svg xmlns="http://www.w3.org/2000/svg" class="spin" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
   <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
   <path d="M3 12a9 9 0 0 0 9 9a9 9 0 0 0 9 -9a9 9 0 0 0 -9 -9"></path>
   <path d="M17 12a5 5 0 1 0 -5 5"></path>
</svg>

<style>
   .spin {
      animation: spin 1s linear infinite;
   }

   @keyframes spin {
   from {
      transform: rotate(0deg);
   }
   to {
      transform: rotate(360deg);
   }
   }
</style>