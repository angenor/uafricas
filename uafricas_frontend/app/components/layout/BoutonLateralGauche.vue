<template>
  <div
    :class="[state.show ? 'left-5' : '-left-24']"
    class="top-40 overflow-show transition-all duration-300 rounded-full border-white border bg-custom-chocolat bg-opacity-80 h-100 w-24 fixed whitespace-nowrap leading-none z-40"
  >
    <div
      @click="state.show = !state.show"
      class="rounded-full hover:text-indigo-800 bg-gray-500 bg-opacity-40 -mt-4 cursor-pointer transition-all duration-300 flex items-center justify-center"
      :class="[
        state.show ? 'ml-6 w-12 h-12' : 'ml-20 w-14 h-14 rotate-180',
      ]">
      <font-awesome-icon
        :icon="state.show ? 'minus' : 'angle-left'"
        class="text-white text-xl"
      />
    </div>
    <!-- ATTOUT -->
    <div
      @click="
        state.pointer == 'attout'
          ? (state.pointer = null)
          : (state.pointer = 'attout')
      "
      class="text-white mt-5 cursor-pointer hover:scale-110 transition-transform"
    >
      <div class="flex justify-center"><font-awesome-icon icon="table-cells-large" class="text-4xl" /></div>
      <div class="text-sm text-center z-20">APPLIS</div>
    </div>

    <!-- JE M'ENGAGE -->
    <div
      @click="
        state.pointer == 'engager'
          ? (state.pointer = null)
          : (state.pointer = 'engager')
      "
      class="text-white mt-5 cursor-pointer hover:scale-110 transition-transform"
    >
      <div><img class="w-10 h-10 ml-6" src="/icons/engage.png" alt="" /></div>
      <div class="text-sm ml-1 z-20">JE M'ENGAGE</div>
    </div>

    <!-- RESEAUTAGE -->
    <div
      @click="
        state.pointer == 'reseautage'
          ? (state.pointer = null)
          : (state.pointer = 'reseautage')
      "
      class="text-white mt-6 cursor-pointer hover:scale-110 transition-transform"
    >
      <div>
        <img class="w-10 h-10 ml-6" src="/icons/innovation.png" alt="" />
      </div>
      <div class="text-sm ml-[0.35rem]">RESEAUTAGE</div>
    </div>

    <!-- SOUMETTRE UN PROJET -->
    <NuxtLink to="/soumettre-projet">
      <div
        @click="state.pointer = null"
        class="text-white mt-6 whitespace-normal cursor-pointer hover:scale-110 transition-transform"
      >
        <div>
          <img class="w-10 h-10 ml-6" src="/icons/soumettre.png" alt="" />
        </div>
        <div class="text-sm ml-1 text-center">SOUMETTRE UN PROJET</div>
      </div>
    </NuxtLink>
  </div>

  <!-- Attout Pop Up -->
  <div
    v-if="state.show"
    :class="[
      state.pointer == 'attout'
        ? 'top-0 left-28 z-50 w-52 transition-all duration-75'
        : 'left-10 z-0 w-0 transition-all duration-500',
    ]"
    class="fixed top-44 h-20 whitespace-nowrap text-custom-chocolat ml-2 bg-white border-t-4 border-custom-chocolat bg-opacity-0"
  >
    <NuxtLink to="/afrolangue/salles-publiques">
      <div
        class="px-2 bg-white bg-opacity-80 py-2 overflow-x-hidden cursor-pointer hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green transition-all duration-300"
      >
        Afrolang
      </div>
    </NuxtLink>
    <NuxtLink to="/africa-culture/codi-moi">
      <div
        :class="[
          state.pointer == 'attout'
            ? 'left-0 duration-100'
            : '-left-44 duration-300',
        ]"
        class="px-2 relative bg-white bg-opacity-80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green"
      >
        Codimoi
      </div>
    </NuxtLink>
    <NuxtLink to="/africain-afro-americain">
      <div
        :class="[
          state.pointer == 'attout'
            ? 'left-0 duration-200'
            : '-left-44 duration-200',
        ]"
        class="px-2 relative bg-white bg-opacity-80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green"
      >
        Afroculture
      </div>
    </NuxtLink>
    <NuxtLink to="/experts/index">
      <div
        :class="[
          state.pointer == 'attout'
            ? 'left-0 duration-300'
            : '-left-44 duration-100',
        ]"
        class="px-2 relative bg-white bg-opacity-80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green"
      >
        Diapertise
      </div>
    </NuxtLink>
    <NuxtLink to="/marche-africain">
      <div
        :class="[
          state.pointer == 'attout'
            ? 'left-0 duration-400'
            : '-left-44 duration-75',
        ]"
        class="px-2 relative bg-white bg-opacity-80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green"
      >
        Afromarket
      </div>
    </NuxtLink>
    <NuxtLink to="/bibliotheque-numerique">
      <div
        :class="[
          state.pointer == 'attout'
            ? 'left-0 duration-500'
            : '-left-44 duration-50',
        ]"
        class="px-2 relative bg-white bg-opacity-80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green"
      >
        Librafrica
      </div>
    </NuxtLink>
    <NuxtLink to="/echanges-sabbatiques">
      <div
        :class="[
          state.pointer == 'attout'
            ? 'left-0 duration-500'
            : '-left-44 duration-50',
        ]"
        class="px-2 relative bg-white bg-opacity-80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green"
      >
        Sabbafrica
      </div>
    </NuxtLink>

    <NuxtLink to="/africantives">
      <div
        :class="[
          state.pointer == 'attout'
            ? 'left-0 duration-500'
            : '-left-44 duration-50',
        ]"
        class="px-2 relative bg-white bg-opacity-80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green"
      >
        Africantives
      </div>
    </NuxtLink>
  </div>

  <!-- S'engager Pup Up -->
  <div
    v-if="state.show"
    :class="[
      state.pointer == 'engager'
        ? 'top-0 left-28 z-50 w-52 transition-all duration-75'
        : 'left-10 z-0 w-0 transition-all duration-500',
    ]"
    class="fixed top-[13.5rem] h-20 whitespace-nowrap text-custom-chocolat ml-2 bg-white border-t-4 border-custom-chocolat bg-opacity-0"
  >
    <NuxtLink to="/apporter-expertise">
      <div
        class="px-2 bg-white bg-opacity-80 py-2 overflow-x-hidden cursor-pointer hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green transition-all duration-300"
      >
        J'apporte mon expertise
      </div>
    </NuxtLink>
    <NuxtLink to="/financer-projet">
      <div
        :class="[
          state.pointer == 'engager'
            ? 'left-0 duration-100'
            : '-left-44 duration-300',
        ]"
        class="px-2 relative bg-white bg-opacity-80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green"
      >
        Je finance un projet
      </div>
    </NuxtLink>
    <NuxtLink to="/partager-innovation">
      <div
        :class="[
          state.pointer == 'engager'
            ? 'left-0 duration-300'
            : '-left-44 duration-100',
        ]"
        class="px-2 relative bg-white bg-opacity-80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green"
      >
        Je partage une innovation
      </div>
    </NuxtLink>
    <NuxtLink to="/devenir-partenaire">
      <div
        :class="[
          state.pointer == 'engager'
            ? 'left-0 duration-500'
            : '-left-44 duration-75',
        ]"
        class="px-2 relative bg-white bg-opacity-80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green"
      >
        Je deviens partenaire
      </div>
    </NuxtLink>
  </div>

  <!-- Reseautage Pup Up -->
  <div
    v-if="state.show"
    :class="[
      state.pointer == 'reseautage'
        ? 'top-0 left-28 z-50 w-46 duration-100'
        : 'left-10 z-0 w-0 duration-500',
    ]"
    class="fixed top-[20.5rem] h-20 whitespace-nowrap text-custom-chocolat ml-2 bg-white border-t-4 border-custom-chocolat bg-opacity-0 transition-all"
  >
    <NuxtLink to="/retrouver-profil">
      <div
        class="px-2 bg-white bg-opacity-80 py-2 overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green"
      >
        Retrouver un profil
      </div>
    </NuxtLink>
    <NuxtLink to="#moboliser_expertise">
      <div
        :class="[
          state.pointer == 'reseautage'
            ? 'left-0 duration-300'
            : '-left-44 duration-100',
        ]"
        class="px-2 relative bg-white bg-opacity-80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green"
      >
        Mobiliser une expertise
      </div>
    </NuxtLink>
    <NuxtLink to="/marche-africain">
      <div
        :class="[
          state.pointer == 'reseautage'
            ? 'left-0 duration-500'
            : '-left-44 duration-100',
        ]"
        class="px-2 relative bg-white bg-opacity-80 py-2 mt-px overflow-x-hidden cursor-pointer transition-all hover:border-l-4 hover:border-custom-green hover:ml-1 hover:bg-opacity-70 hover:text-custom-green"
      >
        Opportunités
      </div>
    </NuxtLink>
  </div>
</template>

<script setup lang="ts">
const route = useRoute()

const isHomeRoute = computed(() =>
  route.name === 'index' || route.path === '/' || route.path === '/home'
)

const state = reactive({
  show: isHomeRoute.value,
  pointer: null as string | null,
})

watch(isHomeRoute, (newValue) => {
  state.show = newValue
})
</script>
