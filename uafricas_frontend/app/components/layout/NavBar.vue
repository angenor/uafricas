<template>
  <div class="entete z-50 h-24 absolute w-full bg-gradient-to-t from-white">
    <NuxtLink to="/">
      <img class="h-14 mx-auto my-1" src="/logos/logo_uafracas.png" alt="UAfricas Logo" />
    </NuxtLink>

    <nav class="w-full flex justify-center space-x-10 h-7 text-custom-chocolat font-semibold">
      <!-- Actions -->
      <div class="relative">
        <NuxtLink
          class="relative cursor-pointer hover:text-custom-green"
          to="/actions"
        >
          Actions
        </NuxtLink>
      </div>

      <!-- AfricaCulture -->
      <div>
        <NuxtLink
          class="relative cursor-pointer hover:text-custom-green"
          to="/africa-culture"
        >
          AfricaCulture
        </NuxtLink>
      </div>

      <!-- Lib. d'Afrique -->
      <div
        @mouseenter="pointer = 'biblio'"
        @mouseleave="pointer = null"
        class="relative"
      >
        <NuxtLink
          to="/bibliotheques"
          :class="pointer === 'biblio' ? 'text-custom-green' : ''"
          class="relative cursor-pointer whitespace-nowrap"
        >
          Lib. d'Afrique
        </NuxtLink>
        <div
          :class="[
            pointer === 'biblio'
              ? 'h-24 border-t-4 border-custom-chocolat top-7'
              : 'h-0 top-5',
          ]"
          class="absolute transition-all duration-500 -left-14 z-50 w-52 whitespace-nowrap text-custom-chocolat ml-2 bg-white/0 overflow-hidden"
        >
          <NuxtLink to="/bibliotheque/numerique">
            <div
              :class="[
                pointer === 'biblio' ? 'top-0 duration-700' : '-top-44 duration-500',
              ]"
              class="px-2 relative bg-white/80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-white/70 hover:text-custom-green hover:duration-100"
            >
              Biblio Numérique
            </div>
          </NuxtLink>
          <NuxtLink to="/bibliotheque/humaine">
            <div
              :class="[
                pointer === 'biblio' ? 'top-0 duration-500' : '-top-40 duration-700',
              ]"
              class="px-2 relative bg-white/80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-white/70 hover:text-custom-green hover:duration-100"
            >
              Biblio Humaine
            </div>
          </NuxtLink>
        </div>
      </div>

      <!-- Africa Univers -->
      <div
        @mouseenter="pointer = 'universite'"
        @mouseleave="pointer = null"
        class="relative"
      >
        <NuxtLink to="/universite" class="relative cursor-pointer whitespace-nowrap">
          Africa Univers
        </NuxtLink>
        <div
          :class="[
            pointer === 'universite'
              ? 'h-44 border-t-4 border-custom-chocolat'
              : 'h-0',
          ]"
          class="top-7 absolute transition-all duration-500 -left-9 z-50 w-52 whitespace-nowrap text-custom-chocolat ml-2 bg-white/0 overflow-hidden"
        >
          <NuxtLink to="/universite/gouvernance">
            <div class="px-2 bg-white/80 py-2 overflow-x-hidden cursor-pointer hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-white/70 hover:text-custom-green transition-all duration-300">
              Gouvernance
            </div>
          </NuxtLink>
          <NuxtLink to="/universite/inuda">
            <div
              :class="[
                pointer === 'universite' ? 'top-0 duration-100' : '-top-44 duration-300',
              ]"
              class="px-2 relative bg-white/80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-white/70 hover:text-custom-green"
            >
              INUDA
            </div>
          </NuxtLink>
        </div>
      </div>

      <!-- Africamood -->
      <div
        @mouseenter="pointer = 'media'"
        @mouseleave="pointer = null"
        class="relative ml-2"
      >
        <NuxtLink to="/medias" class="flex cursor-pointer">
          <div class="text-custom-green">Africamood</div>
          <font-awesome-icon icon="fa-solid fa-tv" class="mt-1 ml-1 text-gray-600" />
        </NuxtLink>
        <div
          :class="[
            pointer === 'media'
              ? 'h-32 border-t-4 border-custom-chocolat top-7'
              : 'h-0 top-5',
          ]"
          class="absolute transition-all duration-500 -left-10 z-50 w-32 whitespace-nowrap text-custom-chocolat ml-2 bg-white/0 overflow-hidden"
        >
          <NuxtLink to="/tele">
            <div
              :class="[
                pointer === 'media' ? 'top-0 duration-700' : '-top-44 duration-500',
              ]"
              class="px-2 relative text-center bg-white/80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-white/70 hover:text-custom-green hover:duration-100"
            >
              Télé
            </div>
          </NuxtLink>
          <NuxtLink to="/radios">
            <div
              :class="[
                pointer === 'media' ? 'top-0 duration-500' : '-top-40 duration-700',
              ]"
              class="px-2 relative text-center bg-white/80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-white/70 hover:text-custom-green hover:duration-100"
            >
              Radio
            </div>
          </NuxtLink>
          <NuxtLink to="/africantives">
            <div
              :class="[
                pointer === 'media' ? 'top-0 duration-500' : '-top-40 duration-700',
              ]"
              class="px-2 relative text-center bg-white/80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-white/70 hover:text-custom-green hover:duration-100"
            >
              Africantives
            </div>
          </NuxtLink>
        </div>
      </div>

      <!-- Bouton Se connecter (mock - pas d'auth pour l'instant) -->
      <NuxtLink
        to="/login"
        class="bg-custom-chocolat text-white px-2 mb-1 whitespace-nowrap rounded-xs"
      >
        Se connecter
      </NuxtLink>
    </nav>
  </div>
</template>

<script setup lang="ts">
const pointer = ref<string | null>(null)
</script>

<style scoped>
.entete {
  box-shadow: 0px 5px 4px rgba(0, 0, 0, 0.5);
}
</style>
